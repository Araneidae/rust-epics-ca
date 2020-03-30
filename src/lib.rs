mod cadef;
mod db_access;
mod dbr;

mod channel;
mod callback;

use async_trait::async_trait;
use cadef::{voidp_to_ref, ref_to_voidp};

pub use std::time::SystemTime;
pub use db_access::StatusSeverity;
pub use db_access::CtrlLimits;


trait GetResult<R: Send, E: Send, D: dbr::Dbr<R, E>>: Send {
    const COUNT: u64;
    fn get_result(dbr: &D, count: usize) -> Self;
}

impl<R, E, D> GetResult<R, E, D> for R
    where R: dbr::DbrMap, E: Send, D: dbr::Dbr<R, E>
{
    const COUNT: u64 = 1;
    fn get_result(dbr: &D, _count: usize) -> Self { dbr.get_value() }
}

impl<R, E, D> GetResult<R, E, D> for Box<[R]>
    where R: dbr::DbrMap, E: Send, D: dbr::Dbr<R, E>
{
    const COUNT: u64 = 0;
    fn get_result(dbr: &D, count: usize) -> Self { dbr.get_value_vec(count) }
}



extern fn caget_callback<R, E, D, T>(args: cadef::event_handler_args)
    where R: Send, E: Send, D: dbr::Dbr<R, E>, T: GetResult<R, E, D>
{
    let waker: &callback::AsyncWaker::<(T, E)> =
        unsafe { voidp_to_ref(args.usr) };
    let dbr: &D = unsafe { voidp_to_ref(args.dbr) };
    let result = (T::get_result(dbr, args.count as usize), dbr.get_extra());
    waker.wake(result);
}

async fn do_caget<R, E, D, T>(pv: &str) -> (T, E)
    where R: Send, E: Send, D: dbr::Dbr<R, E>, T: GetResult<R, E, D>
{
    let (channel, _datatype, _count) = channel::connect(pv).await;

    let waker = callback::AsyncWaker::<(T, E)>::new();
    let rc = unsafe { cadef::ca_array_get_callback(
        D::DATATYPE as i64, T::COUNT, channel.id,
        caget_callback::<R, E, D, T>, ref_to_voidp(&waker)) };
    assert!(rc == 1);
    unsafe { cadef::ca_flush_io() };
    waker.wait_for().await
}


#[async_trait(?Send)]
pub trait CA {
    async fn caget(pv: &str) -> Self;
}

#[async_trait(?Send)]
impl<T> CA for T where T: dbr::DbrMap {
    async fn caget(pv: &str) -> Self {
        do_caget::<_, _, T::ValueDbr, _>(pv).await.0
    }
}

#[async_trait(?Send)]
impl<T> CA for Box<[T]> where T: dbr::DbrMap {
    async fn caget(pv: &str) -> Self {
        do_caget::<_, _, T::ValueDbr, _>(pv).await.0
    }
}

#[async_trait(?Send)]
impl<T> CA for (T, StatusSeverity, SystemTime) where T: dbr::DbrMap {
    async fn caget(pv: &str) -> Self {
        let (v, (s, t)) = do_caget::<_, _, T::TimeDbr, _>(pv).await;
        (v, s, t)
    }
}

#[async_trait(?Send)]
impl<T> CA for (Box<[T]>, StatusSeverity, SystemTime) where T: dbr::DbrMap {
    async fn caget(pv: &str) -> Self {
        let (v, (s, t)) = do_caget::<_, _, T::TimeDbr, _>(pv).await;
        (v, s, t)
    }
}

#[async_trait(?Send)]
impl<T> CA for (T, T::CtrlType) where T: dbr::DbrMap {
    async fn caget(pv: &str) -> Self {
        do_caget::<_, _, T::CtrlDbr, _>(pv).await
    }
}

#[async_trait(?Send)]
impl<T> CA for (Box<[T]>, T::CtrlType) where T: dbr::DbrMap {
    async fn caget(pv: &str) -> Self {
        do_caget::<_, _, T::CtrlDbr, _>(pv).await
    }
}
