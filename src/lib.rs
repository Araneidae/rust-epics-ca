mod cadef;
mod dbr;

mod channel;
mod callback;

use async_trait::async_trait;
use cadef::{voidp_to_ref, ref_to_voidp};
use libc;


extern fn caget_callback<T>(args: cadef::event_handler_args)
    where T: dbr::Adapter + Send + Copy
{
    let caget_waker: &callback::AsyncWaker::<T> =
        unsafe { voidp_to_ref(args.usr) };
    let dbr: &T::DbrType = unsafe { voidp_to_ref(args.dbr) };
    caget_waker.wake(*T::get_value(dbr));
}

async fn do_caget<T>(pv: &str) -> T
    where T: dbr::Adapter + Send + Copy
{
    let (channel, _datatype, count) = channel::connect(pv).await;

    let caget_waker = callback::AsyncWaker::<T>::new();
    let rc = unsafe { cadef::ca_array_get_callback(
        T::DATATYPE as libc::c_long, count as u64, channel.id,
        caget_callback::<T>, ref_to_voidp(&caget_waker)) };
    assert!(rc == 1);
    unsafe { cadef::ca_flush_io() };
    caget_waker.wait_for().await
}


// Note the ?Send here.  This means that the async function is not thread safe.
#[async_trait(?Send)]
pub trait CA: dbr::Adapter {
    async fn caget(pv: &str) -> Self;
    async fn caput(pv: &str, t: Self);
}

#[async_trait(?Send)]
impl<T> CA for T where T: dbr::Adapter + Send + Copy {
    async fn caget(pv: &str) -> Self { do_caget(pv).await }
    async fn caput(_pv: &str, _t: Self) { panic!("Not implemented"); }
}
