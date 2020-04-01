// Implementation of caget functionality

use async_trait::async_trait;

use crate::cadef;
use crate::dbr;
use crate::callback;
use crate::channel;
use crate::caunion;

use std::time::SystemTime;
use crate::db_access::StatusSeverity;


// Overloaded trait for returning the underlying Dbr value using one of the two
// Dbr readout methods: get_value or get_value_vec.
//
// Here R is the underlying datatype associated with the Dbr D, and E is the
// extra (timestamp+severity or control) data for the Dbr.  The implementing
// datatype Self will be either R or Vec<R>.
pub trait GetResult<D: dbr::Dbr>: Send {
    const COUNT: u64;
    fn get_result(dbr: &D, count: usize) -> Self;
}


impl<R, D> GetResult<D> for R
    where R: dbr::DbrMap, D: dbr::Dbr<ResultType=R>
{
    const COUNT: u64 = 1;
    fn get_result(dbr: &D, _count: usize) -> Self { dbr.get_value() }
}

impl<R, D> GetResult<D> for Vec<R>
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
        unsafe { cadef::voidp_to_ref(args.usr) };
    let dbr: &D = unsafe { cadef::voidp_to_ref(args.dbr) };
    let result = (T::get_result(dbr, args.count as usize), dbr.get_extra());
    waker.wake(result);
}


async fn caget_core<D, T>(channel: &channel::Channel) -> (T, D::ExtraType)
    where D: dbr::Dbr, T: GetResult<D>
{
    let waker = callback::AsyncWaker::<(T, D::ExtraType)>::new();
    let rc = unsafe { cadef::ca_array_get_callback(
        D::DATATYPE as i64, T::COUNT, channel.id,
        caget_callback::<D, T>, cadef::ref_to_voidp(&waker)) };
    assert!(rc == 1);
    unsafe { cadef::ca_flush_io() };
    waker.wait_for().await
}


// -----------------------------------------------------------------------------
// Implementation of caget_core for all of the basic target types

#[async_trait(?Send)]
pub trait CaGetCore {
    async fn caget_core(channel: &channel::Channel) -> Self;
}



// caget_core of undecorated value, either as scalar or vector

#[async_trait(?Send)]
impl<T> CaGetCore for T where T: dbr::DbrMap {
    async fn caget_core(channel: &channel::Channel) -> Self {
        caget_core::<T::ValueDbr, _>(channel).await.0
    }
}

#[async_trait(?Send)]
impl<T> CaGetCore for Vec<T> where T: dbr::DbrMap {
    async fn caget_core(channel: &channel::Channel) -> Self {
        caget_core::<T::ValueDbr, _>(channel).await.0
    }
}


// caget_core with severity and timestamp

#[async_trait(?Send)]
impl<T> CaGetCore for (T, StatusSeverity, SystemTime) where T: dbr::DbrMap {
    async fn caget_core(channel: &channel::Channel) -> Self {
        let (v, (s, t)) = caget_core::<T::TimeDbr, _>(channel).await;
        (v, s, t)
    }
}

#[async_trait(?Send)]
impl<T> CaGetCore for (Vec<T>, StatusSeverity, SystemTime)
    where T: dbr::DbrMap
{
    async fn caget_core(channel: &channel::Channel) -> Self {
        let (v, (s, t)) = caget_core::<T::TimeDbr, _>(channel).await;
        (v, s, t)
    }
}


// caget_core with control field information

#[derive(Clone, Copy, Debug)]
pub struct CaCtrl<T>(pub T);

#[async_trait(?Send)]
impl<T> CaGetCore for (T, StatusSeverity, CaCtrl<T::CtrlType>)
    where T: dbr::DbrMap
{
    async fn caget_core(channel: &channel::Channel) -> Self {
        let (v, (s, c)) = caget_core::<T::CtrlDbr, _>(channel).await;
        (v, s, CaCtrl(c))
    }
}

#[async_trait(?Send)]
impl<T> CaGetCore for (Vec<T>, StatusSeverity, CaCtrl<T::CtrlType>)
    where T: dbr::DbrMap
{
    async fn caget_core(channel: &channel::Channel) -> Self {
        let (v, (s, c)) = caget_core::<T::CtrlDbr, _>(channel).await;
        (v, s, CaCtrl(c))
    }
}


// -----------------------------------------------------------------------------
// caget

#[async_trait(?Send)]
pub trait CA {
    async fn caget(pv: &str) -> Self;
}

#[async_trait(?Send)]
impl<T> CA for T where T: CaGetCore {
    async fn caget(pv: &str) -> Self {
        let (channel, _datatype, _count) = channel::connect(pv).await;
        T::caget_core(&channel).await
    }
}

#[async_trait(?Send)]
impl CA for caunion::CaUnion {
    async fn caget(pv: &str) -> Self {
        caunion::caget_union(pv).await
    }
}
