mod cadef;

use std::sync::Mutex;
use libc::c_void;
use crate::cadef::*;


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


#[allow(unused_unsafe)]
unsafe fn voidp_to_ref<'a, T>(p: *const c_void) -> &'a T
{
    unsafe { &*(p as *const T) }
}

fn ref_to_voidp<T>(r: &T) -> *const c_void
{
    r as *const T as *const c_void
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
                    // Treat this as disconnected.  Don't actually know if this
                    // can happen, depends on how well the connection callback
                    // is synchronised with the channel state.
                    println!("Failed to read {:?}", x);
                    ChannelState::Disconnected
                }
            }
        },
        CA_OP_CONN_DOWN => {
            ChannelState::Disconnected
        },
        x =>
            panic!("Unexpected connection state {}", x),
    };
    println!("state: {:?}", state);
    *channel.state.lock().unwrap() = state;
}

impl Channel {
    pub fn new(pv: &str) -> Box<Channel>
    {
        context_create();

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



// Code to ensure that the context is valid
static CA_CONTEXT_CREATE: std::sync::Once = std::sync::Once::new();
fn context_create()
{
    CA_CONTEXT_CREATE.call_once(|| {
        println!("Calling ca_context_create");
        unsafe { ca_context_create(
            ca_preemptive_callback_select::ca_enable_preemptive_callback) };
    });
}
