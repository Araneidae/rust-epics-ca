use libc::{c_char, c_int, c_long, c_uint, c_void};

#[link(name = "ca")]
extern {
    fn ca_context_create(
        select: ca_preemptive_callback_select) -> c_int;
    fn ca_create_channel(
        pv: *const c_char,
        on_connect : extern fn(args: ca_connection_handler_args),
        context: *mut c_void,
        priority: c_uint,
        channel_id: *mut ChanId) -> c_int;
    fn ca_puser(channel: ChanId) -> *mut c_void;
    pub fn ca_pend_event(timeout : f64) -> c_int;
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]     // For unused variant
enum ca_preemptive_callback_select {
    ca_disable_preemptive_callback,
    ca_enable_preemptive_callback,
}

#[repr(C)]
struct ca_connection_handler_args {
    chid: ChanId,
    op: c_long,
}

#[repr(C)]
#[derive(Debug)]
struct oldChannelNotify { _unused: [u8; 0] }
type ChanId = *mut oldChannelNotify;


pub fn safe_context_create()
{
    unsafe { ca_context_create(
        ca_preemptive_callback_select::ca_disable_preemptive_callback) };
}


extern fn on_connect(args: ca_connection_handler_args)
{
    println!("Hello there: {:?} {}", args.chid, args.op);
    let context = unsafe { ca_puser(args.chid) };
    println!("Context: {:?}", context);
}

pub fn create_channel(pv : &str)
{
    let cpv = std::ffi::CString::new(pv).unwrap();
    let mut chan_id = std::mem::MaybeUninit::uninit();
    let rc = unsafe {
        ca_create_channel(
            cpv.as_ptr(), on_connect, std::ptr::null_mut(), 0,
            chan_id.as_mut_ptr()) };
    let chan_id = unsafe { chan_id.assume_init() };
    println!("ca_create_channel => {:} {:?}", rc, chan_id);
}
