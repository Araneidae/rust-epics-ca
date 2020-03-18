use libc;

#[link(name = "ca")]
extern {
    pub fn ca_context_create(
        select: ca_preemptive_callback_select) -> libc::c_int;
    pub fn ca_create_channel(
        pv: *const libc::c_char,
        on_connect : extern fn(args: ca_connection_handler_args),
        context: *const libc::c_void,
        priority: libc::c_uint,
        id: *mut ChanId) -> libc::c_int;
    pub fn ca_clear_channel(id: ChanId) -> libc::c_int;
    pub fn ca_puser(channel: ChanId) -> *const libc::c_void;
    pub fn ca_field_type(channel: ChanId) -> libc::c_short;
    pub fn ca_element_count(channel: ChanId) -> libc::c_ulong;
    pub fn ca_pend_event(timeout : f64) -> libc::c_int;
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]     // For unused variant
pub enum ca_preemptive_callback_select {
    ca_disable_preemptive_callback,
    ca_enable_preemptive_callback,
}

#[repr(C)]
#[derive(Debug)]
pub struct ca_connection_handler_args {
    pub chid: ChanId,
    pub op: libc::c_long,
}

pub const CA_OP_CONN_UP: libc::c_long = 6;
pub const CA_OP_CONN_DOWN: libc::c_long = 7;

#[repr(C)]
#[derive(Debug)]
pub struct oldChannelNotify { _unused: [u8; 0] }
pub type ChanId = *const oldChannelNotify;
