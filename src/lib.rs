use libc::{c_char, c_int, c_long, c_uint, c_void};

#[link(name = "ca")]
extern {
    fn ca_context_create(
        select: ca_preemptive_callback_select) -> c_int;
    fn ca_create_channel(
        pv: *const c_char,
        on_connect : extern fn(args: ca_connection_handler_args),
        context: *const c_void,
        priority: c_uint,
        id: *mut ChanId) -> c_int;
    fn ca_clear_channel(id: ChanId) -> c_int;
    fn ca_puser(channel: ChanId) -> *const c_void;
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
#[derive(Debug)]
struct ca_connection_handler_args {
    chid: ChanId,
    op: c_long,
}

#[repr(C)]
#[derive(Debug)]
struct oldChannelNotify { _unused: [u8; 0] }
type ChanId = *const oldChannelNotify;


pub fn context_create()
{
    unsafe { ca_context_create(
        ca_preemptive_callback_select::ca_disable_preemptive_callback) };
}


#[derive(Debug)]
pub struct Channel {
    name: String,
    id: ChanId,
}


extern fn on_connect(args: ca_connection_handler_args)
{
    println!("Processing callback: {:?}", args);
    let channel: &Channel = unsafe {
        &*(ca_puser(args.chid) as *const Channel) };
    println!("Channel: {:?}", channel);
}

impl Channel {
    pub fn new(pv: &str) -> Box<Channel>
    {
        let mut channel = Box::new(Channel {
            name: pv.to_owned(),
            id: 0 as _,
        });

        let cpv = std::ffi::CString::new(pv).unwrap();
        let mut chan_id = 0 as ChanId;
        let rc = unsafe {
            ca_create_channel(
                cpv.as_ptr(),
                on_connect,
                channel.as_ref() as *const _ as *const c_void,
                0,
                &mut chan_id as *mut ChanId) };
        assert!(rc == 1);

        channel.id = chan_id;
        channel
    }
}

impl Drop for Channel {
    fn drop(self: &mut Channel)
    {
        println!("Dropping {:?}", self);
        let rc = unsafe { ca_clear_channel(self.id) };
        assert!(rc == 1);
    }
}
