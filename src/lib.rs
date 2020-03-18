use std::sync::Mutex;
use libc;

#[link(name = "ca")]
extern {
    fn ca_context_create(
        select: ca_preemptive_callback_select) -> libc::c_int;
    fn ca_create_channel(
        pv: *const libc::c_char,
        on_connect : extern fn(args: ca_connection_handler_args),
        context: *const libc::c_void,
        priority: libc::c_uint,
        id: *mut ChanId) -> libc::c_int;
    fn ca_clear_channel(id: ChanId) -> libc::c_int;
    fn ca_puser(channel: ChanId) -> *const libc::c_void;
    fn ca_field_type(channel: ChanId) -> libc::c_short;
    fn ca_element_count(channel: ChanId) -> libc::c_ulong;
    pub fn ca_pend_event(timeout : f64) -> libc::c_int;
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
    op: libc::c_long,
}

const CA_OP_CONN_UP: libc::c_long = 6;
const CA_OP_CONN_DOWN: libc::c_long = 7;

#[repr(C)]
#[derive(Debug)]
struct oldChannelNotify { _unused: [u8; 0] }
type ChanId = *const oldChannelNotify;

#[derive(Debug)]
enum BasicDbrType {
    DbrString,
    DbrShort,
    DbrFloat,
    DbrEnum,
    DbrChar,
    DbrLong,
    DbrDouble,
}


pub fn context_create()
{
    unsafe { ca_context_create(
        ca_preemptive_callback_select::ca_disable_preemptive_callback) };
}


#[allow(unused_unsafe)]
unsafe fn voidp_to_ref<'a, T>(p: *const libc::c_void) -> &'a T
{
    unsafe { &*(p as *const T) }
}

fn ref_to_voidp<T>(r: &T) -> *const libc::c_void
{
    r as *const T as *const libc::c_void
}


#[derive(Debug)]
enum ChannelState {
    Unconnected,            // Connect event never seen
    Disconnected,           // Channel disconnected
    Connected {
        field_type: BasicDbrType,
        field_count: usize,
    },
}

#[derive(Debug)]
pub struct Channel {
    name: String,
    id: ChanId,
    state: Mutex<ChannelState>,
}


fn get_field_type(id: ChanId) -> Option<BasicDbrType>
{
    match unsafe { ca_field_type(id) } {
        0 => Some(BasicDbrType::DbrString),
        1 => Some(BasicDbrType::DbrShort),
        2 => Some(BasicDbrType::DbrFloat),
        3 => Some(BasicDbrType::DbrEnum),
        4 => Some(BasicDbrType::DbrChar),
        5 => Some(BasicDbrType::DbrLong),
        6 => Some(BasicDbrType::DbrDouble),
        // None of the above.  Probably unexpectedly disconnected
        _ => None,
    }
}

fn get_element_count(id: ChanId) -> Option<usize>
{
    let count = unsafe { ca_element_count(id) };
    if count <= 0 {
        // Treat this as disconnected
        None
    } else {
        Some(count as usize)
    }
}


extern fn on_connect(args: ca_connection_handler_args)
{
    let channel: &Channel = unsafe { voidp_to_ref(ca_puser(args.chid)) };
    println!("on_connect: {} {:?}", args.op, channel);
    let state = match args.op {
        CA_OP_CONN_UP => {
            match (get_field_type(args.chid), get_element_count(args.chid))
            {
                (Some(field_type), Some(field_count)) =>
                    ChannelState::Connected { field_type, field_count },
                x => {
                    // Treat this as disconnected
                    println!("Failed to read {:?}", x);
                    ChannelState::Disconnected
                }
            }
        },
        CA_OP_CONN_DOWN => {
            ChannelState::Disconnected
        },
        x =>
            panic!("Unexpected state {}", x),
    };
    println!("state: {:?}", state);
    *channel.state.lock().unwrap() = state;
}

impl Channel {
    pub fn new(pv: &str) -> Box<Channel>
    {
        let mut channel = Box::new(Channel {
            name: pv.to_owned(),
            id: 0 as ChanId,
            state: Mutex::new(ChannelState::Unconnected),
        });

        let cpv = std::ffi::CString::new(pv).unwrap();
        let mut chan_id = 0 as ChanId;
        let rc = unsafe {
            ca_create_channel(
                cpv.as_ptr(), on_connect, ref_to_voidp(channel.as_ref()),
                0, &mut chan_id as *mut ChanId) };
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
