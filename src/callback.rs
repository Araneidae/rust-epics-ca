// Simple async callback helper

use std::{sync, future, pin, task};


enum WakerState<T> {
    Idle,
    Waiting(task::Waker),
    Ready(T),
}

pub struct AsyncWaker<T: Sync> {
    wakeup: sync::Mutex<WakerState<T>>,
}

impl<T: Sync> AsyncWaker<T> {
    pub fn new() -> AsyncWaker<T>
    {
        AsyncWaker { wakeup: sync::Mutex::new(WakerState::Idle) }
    }

    pub fn wake(&self, result: T)
    {
        let mut wakeup = self.wakeup.lock().unwrap();
        let mut current_state = WakerState::Idle;
        std::mem::swap(&mut *wakeup, &mut current_state);

        if let WakerState::Waiting(waker) = current_state {
            *wakeup = WakerState::Ready(result);
            waker.wake();
        } else {
            println!("Hmm.  Unexpected wakeup state");
        }
    }
}


struct Waiter<'a, T: Sync> {
    wakeup: &'a AsyncWaker<T>,
}

impl<'a, T: Sync> Waiter<'a, T> {
    fn new(wakeup: &'a AsyncWaker<T>) -> Waiter<'a, T>
    {
        Waiter { wakeup }
    }
}


impl<'a, T: Sync> future::Future for Waiter<'a, T> {
    type Output = T;

    fn poll(self: pin::Pin<&mut Self>, context: &mut task::Context)
        -> task::Poll<Self::Output>
    {
        let mut wakeup = self.wakeup.wakeup.lock().unwrap();
        let mut current_state = WakerState::Idle;
        std::mem::swap(&mut *wakeup, &mut current_state);
        match current_state {
            WakerState::Idle => {
                *wakeup = WakerState::Waiting(context.waker().clone());
                task::Poll::Pending
            },
            WakerState::Ready(result) => {
                *wakeup = WakerState::Idle;
                task::Poll::Ready(result)
            },
            WakerState::Waiting(_) => {
                panic!("Already waiting!");
            }
        }
    }
}

impl<T: Sync> AsyncWaker<T> {
    pub async fn wait_for(&self) -> T
    {
        Waiter::new(self).await
    }
}
