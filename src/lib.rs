mod cadef;
mod dbr;

mod channel;
mod callback;

use cadef::{voidp_to_ref, ref_to_voidp};
use libc;


extern fn caget_event_handler(args: cadef::event_handler_args)
{
    println!("caget callback: {:?}", args);
    let waker: &callback::AsyncWaker::<f64> = unsafe { voidp_to_ref(args.usr) };

    assert!(args.datatype == 6);
    let data: &dbr::dbr_double = unsafe { voidp_to_ref(args.dbr) };

    waker.wake(data.value);
}


pub async fn caget(pv: &str) -> f64
{
    let channel = channel::Channel::new(pv);
    let (datatype, count) = channel.wait_connect().await;
    println!("Got {:?} {:?} {}", channel, datatype, count);

    let caget_result = callback::AsyncWaker::<f64>::new();
    let rc = unsafe { cadef::ca_array_get_callback(
        datatype as libc::c_long, count as u64, channel.id, caget_event_handler,
        ref_to_voidp(&caget_result)) };
    assert!(rc == 1);
    unsafe { cadef::ca_flush_io() };
    caget_result.wait_for().await
}
