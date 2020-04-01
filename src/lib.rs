mod cadef;
mod db_access;
mod dbr;

mod channel;
mod callback;

use async_trait::async_trait;
use cadef::{voidp_to_ref, ref_to_voidp};

pub use std::time::SystemTime;
pub use db_access::{StatusSeverity, CtrlLimits};
pub use dbr::CaEnum;


// Overloaded trait for returning the underlying Dbr value using one of the two
// Dbr readout methods: get_value or get_value_vec.
//
// Here R is the underlying datatype associated with the Dbr D, and E is the
// extra (timestamp+severity or control) data for the Dbr.  The implementing
// datatype Self will be either R or Vec<R>.
trait GetResult<D: dbr::Dbr>: Send {
    const COUNT: u64;
    fn get_result(dbr: &D, count: usize) -> Self;
}


impl<R, D> GetResult<D> for R
    where R: dbr::DbrMap, D: dbr::Dbr<ResultType=R>
{
    const COUNT: u64 = 1;
    fn get_result(dbr: &D, _count: usize) -> Self { dbr.get_value() }
}

impl<R, D> GetResult<D> for Box<[R]>
    where R: dbr::DbrMap, D: dbr::Dbr<ResultType=R>
{
    const COUNT: u64 = 0;
    fn get_result(dbr: &D, count: usize) -> Self { dbr.get_value_vec(count) }
}



// Asynchronous callback invoked in response to ca_array_get_callback.  The two
// type parameters are as follows:
//
//  D: the Dbr type for the returned data
//  T: the actual type we're going to return, supported by GetResult<D>.
//     In practice, this type is either D::ResultType or Vec<D::ResultType>.
extern fn caget_callback<D, T>(args: cadef::event_handler_args)
    where D: dbr::Dbr, T: GetResult<D>
{
    let waker: &callback::AsyncWaker::<(T, D::ExtraType)> =
        unsafe { voidp_to_ref(args.usr) };
    let dbr: &D = unsafe { voidp_to_ref(args.dbr) };
    let result = (T::get_result(dbr, args.count as usize), dbr.get_extra());
    waker.wake(result);
}

async fn do_caget<D, T>(pv: &str) -> (T, D::ExtraType)
    where D: dbr::Dbr, T: GetResult<D>
{
    let (channel, _datatype, _count) = channel::connect(pv).await;

    let waker = callback::AsyncWaker::<(T, D::ExtraType)>::new();
    let rc = unsafe { cadef::ca_array_get_callback(
        D::DATATYPE as i64, T::COUNT, channel.id,
        caget_callback::<D, T>, ref_to_voidp(&waker)) };
    assert!(rc == 1);
    unsafe { cadef::ca_flush_io() };
    waker.wait_for().await
}


// -----------------------------------------------------------------------------
// caget

// This is the overloaded trait used to implement caget.  We have six separate
// implementations for each of the available datatypes.
#[async_trait(?Send)]
pub trait CA {
    async fn caget(pv: &str) -> Self;
}


// caget of undecorated value, either as scalar or vector

#[async_trait(?Send)]
impl<T> CA for T where T: dbr::DbrMap {
    async fn caget(pv: &str) -> Self {
        do_caget::<T::ValueDbr, _>(pv).await.0
    }
}

#[async_trait(?Send)]
impl<T> CA for Box<[T]> where T: dbr::DbrMap {
    async fn caget(pv: &str) -> Self {
        do_caget::<T::ValueDbr, _>(pv).await.0
    }
}


// caget with severity and timestamp

#[async_trait(?Send)]
impl<T> CA for (T, StatusSeverity, SystemTime) where T: dbr::DbrMap {
    async fn caget(pv: &str) -> Self {
        let (v, (s, t)) = do_caget::<T::TimeDbr, _>(pv).await;
        (v, s, t)
    }
}

#[async_trait(?Send)]
impl<T> CA for (Box<[T]>, StatusSeverity, SystemTime) where T: dbr::DbrMap {
    async fn caget(pv: &str) -> Self {
        let (v, (s, t)) = do_caget::<T::TimeDbr, _>(pv).await;
        (v, s, t)
    }
}


// caget with control field information

#[derive(Clone, Copy, Debug)]
pub struct CaCtrl<T>(pub T);

#[async_trait(?Send)]
impl<T> CA for (T, StatusSeverity, CaCtrl<T::CtrlType>) where T: dbr::DbrMap {
    async fn caget(pv: &str) -> Self {
        let (v, (s, c)) = do_caget::<T::CtrlDbr, _>(pv).await;
        (v, s, CaCtrl(c))
    }
}

#[async_trait(?Send)]
impl<T> CA for (Box<[T]>, StatusSeverity, CaCtrl<T::CtrlType>)
    where T: dbr::DbrMap
{
    async fn caget(pv: &str) -> Self {
        let (v, (s, c)) = do_caget::<T::CtrlDbr, _>(pv).await;
        (v, s, CaCtrl(c))
    }
}
