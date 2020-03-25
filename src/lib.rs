mod cadef;
mod dbr;

mod channel;
mod callback;

use async_trait::async_trait;
use cadef::{voidp_to_ref, ref_to_voidp};
use libc;


extern fn caget_callback<T>(args: cadef::event_handler_args)
    where T: dbr::Adapter
{
    let caget_waker: &callback::AsyncWaker::<T> =
        unsafe { voidp_to_ref(args.usr) };
    let dbr: &T::DbrType = unsafe { voidp_to_ref(args.dbr) };
    caget_waker.wake(T::get_value(dbr));
}

async fn do_caget<T>(pv: &str) -> T
    where T: dbr::Adapter
{
    let (channel, _datatype, _count) = channel::connect(pv).await;

    let caget_waker = callback::AsyncWaker::<T>::new();
    let rc = unsafe { cadef::ca_array_get_callback(
        T::DATATYPE as libc::c_long, 1, channel.id,
        caget_callback::<T>, ref_to_voidp(&caget_waker)) };
    assert!(rc == 1);
    unsafe { cadef::ca_flush_io() };
    caget_waker.wait_for().await
}


extern fn caget_vec_callback<T>(args: cadef::event_handler_args)
    where T: dbr::Adapter
{
    let caget_waker: &callback::AsyncWaker::<Vec<T>> =
        unsafe { voidp_to_ref(args.usr) };
    let dbr: &T::DbrType = unsafe { voidp_to_ref(args.dbr) };
    caget_waker.wake(T::get_value_vec(dbr, args.count as usize));
}

async fn do_caget_vec<T>(pv: &str) -> Vec<T>
    where T: dbr::Adapter
{
    let (channel, _datatype, _count) = channel::connect(pv).await;

    let caget_waker = callback::AsyncWaker::<Vec<T>>::new();
    let rc = unsafe { cadef::ca_array_get_callback(
        T::DATATYPE as libc::c_long, 0, channel.id,
        caget_vec_callback::<T>, ref_to_voidp(&caget_waker)) };
    assert!(rc == 1);
    unsafe { cadef::ca_flush_io() };
    caget_waker.wait_for().await
}



// Note the ?Send here.  This means that the async function is not thread safe.
#[async_trait(?Send)]
pub trait CA: dbr::Adapter {
    async fn caget(pv: &str) -> Self;
    async fn caget_vec(pv: &str) -> Vec<Self>;
    async fn caput(pv: &str, t: Self);
}

#[async_trait(?Send)]
impl<T> CA for T where T: dbr::Adapter {
    async fn caget(pv: &str) -> Self { do_caget(pv).await }
    async fn caget_vec(pv: &str) -> Vec<Self> { do_caget_vec(pv).await }
    async fn caput(_pv: &str, _t: Self) { panic!("Not implemented"); }
}
