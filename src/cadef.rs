use libc::{c_char, c_short, c_int, c_uint, c_long, c_ulong, c_void};

// Entry points from cadef.h
#[link(name = "ca")]
extern "C" {
    pub fn ca_context_create(
        select: ca_preemptive_callback_select) -> c_int;
    pub fn ca_create_channel(
        pv: *const c_char,
        on_connect : extern fn(args: ca_connection_handler_args),
        context: *const c_void,
        priority: c_uint,
        id: *mut ChanId) -> c_int;
    pub fn ca_clear_channel(id: ChanId) -> c_int;
    pub fn ca_puser(channel: ChanId) -> *const c_void;
    pub fn ca_field_type(channel: ChanId) -> c_short;
    pub fn ca_element_count(channel: ChanId) -> c_ulong;
    pub fn ca_array_get_callback(
        channel_type: c_long, count: c_ulong, channel: ChanId,
        handler: extern fn(args: event_handler_args),
        context: *const c_void) -> c_int;
    pub fn ca_flush_io() -> c_int;
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum ca_preemptive_callback_select {
    ca_disable_preemptive_callback,
    ca_enable_preemptive_callback,
}

#[repr(C)]
#[derive(Debug)]
pub struct ca_connection_handler_args {
    pub chid: ChanId,
    pub op: c_long,
}

#[repr(C)]
#[derive(Debug)]
pub struct event_handler_args {
    pub usr: *const c_void,
    pub channel: ChanId,
    pub datatype: c_long,
    pub count: c_long,
    pub dbr: *const c_void,
    pub status: c_int,
}

// Valid values for ca_connection_handler_args::op
pub const CA_OP_CONN_UP: c_long = 6;
pub const CA_OP_CONN_DOWN: c_long = 7;

// Opaque channel identifier
#[repr(transparent)]
#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct ChanId(*const c_void);
unsafe impl Send for ChanId { }

pub const CHAN_ID_VOID: ChanId = ChanId(0 as _);


// Helper methods for void* conversion

#[allow(unused_unsafe)]
pub unsafe fn voidp_to_ref<'a, T>(p: *const c_void) -> &'a T
{
    unsafe { &*(p as *const T) }
}

pub fn ref_to_voidp<T>(r: &T) -> *const c_void
{
    r as *const T as *const c_void
}
