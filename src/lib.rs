mod cadef;

use std::{ffi, sync, future, pin, task};
use crate::cadef::*;


#[derive(Debug, Clone, Copy)]
pub enum BasicDbrType {
    DbrString,
    DbrShort,
    DbrFloat,
    DbrEnum,
    DbrChar,
    DbrLong,
    DbrDouble,
}


#[derive(Debug)]
enum ChannelConnection {
    Unconnected,            // Connect event never seen
    Disconnected,           // Channel disconnected
    Connected(BasicDbrType, usize),
}

#[derive(Debug)]
struct ChannelState {
    connection: ChannelConnection,
    wakers: Vec<task::Waker>,
}

#[derive(Debug)]
pub struct Channel {
    name: String,
    id: ChanId,
    state: sync::Mutex<ChannelState>,
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


// Called whenever the associated channel connection state changes.
extern fn on_connect(args: ca_connection_handler_args)
{
    let channel: &Channel = unsafe { voidp_to_ref(ca_puser(args.chid)) };
    println!("on_connect: {} {:?}", args.op, channel);
    let mut connected = false;
    let connection = match args.op {
        CA_OP_CONN_UP => {
            match (get_field_type(args.chid), get_element_count(args.chid))
            {
                (Some(field_type), Some(field_count)) => {
                    connected = true;
                    ChannelConnection::Connected(field_type, field_count)
                },
                x => {
                    // Treat this as disconnected.  Don't actually know if this
                    // can happen, depends on how well the connection callback
                    // is synchronised with the channel state.
                    println!("Failed to read {:?}", x);
                    ChannelConnection::Disconnected
                }
            }
        },
        CA_OP_CONN_DOWN => {
            ChannelConnection::Disconnected
        },
        x => {
            println!("Unexpected connection state {}", x);
            ChannelConnection::Disconnected
        },
    };
    println!("connection: {:?}", connection);
    let mut state = channel.state.lock().unwrap();
    state.connection = connection;
    if connected {
        for waker in state.wakers.drain(..) {
            println!("Calling waker");
            waker.wake();
        }
    }
}


impl Channel {
    pub fn new(pv: &str) -> Box<Channel>
    {
        context_create();

        let mut channel = Box::new(Channel {
            name: pv.to_owned(),
            id: 0 as ChanId,
            state: sync::Mutex::new(ChannelState {
                connection: ChannelConnection::Unconnected,
                wakers: Vec::new(),
            }),
        });

        let cpv = ffi::CString::new(pv).unwrap();
        let mut chan_id = 0 as ChanId;
        let rc = unsafe {
            ca_create_channel(
                cpv.as_ptr(), on_connect, ref_to_voidp(channel.as_ref()),
                0, &mut chan_id as *mut ChanId) };
        assert!(rc == 1);

        channel.id = chan_id;
        channel
    }

    pub async fn wait_connect(&self) -> (BasicDbrType, usize)
    {
        println!("Waiting for {:?}", self);
        ChannelWait::new(self).await
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


// Helper for waiting for a channel
struct ChannelWait<'a> {
    channel : &'a Channel,
}

impl<'a> ChannelWait<'a> {
    fn new(channel: &Channel) -> ChannelWait
    {
        ChannelWait { channel }
    }
}


impl<'a> future::Future for ChannelWait<'a> {
    type Output = (BasicDbrType, usize);

    fn poll(self: pin::Pin<&mut Self>, context: &mut task::Context)
        -> task::Poll<Self::Output>
    {
        let mut state = self.channel.state.lock().unwrap();
        if let ChannelConnection::Connected(field_type, field_count) =
            state.connection {
            task::Poll::Ready((field_type, field_count))
        } else {
            println!("Pushing waker");
            state.wakers.push(context.waker().clone());
            task::Poll::Pending
        }
    }
}



// Code to ensure that the context is valid
static CA_CONTEXT_CREATE: sync::Once = sync::Once::new();
fn context_create()
{
    CA_CONTEXT_CREATE.call_once(|| {
        println!("Calling ca_context_create");
        unsafe { ca_context_create(
            ca_preemptive_callback_select::ca_enable_preemptive_callback) };
    });
}
