// Channel implementation

use std::{ffi, sync, future, pin, task};
use static_assertions::*;

use crate::cadef as cadef;
use crate::cadef::{ChanId, ref_to_voidp, voidp_to_ref};
use crate::caunion;
use crate::caunion::BasicDbrType;


// When we have a connected channel we snapshot the underlying data type and
// data size on connection.
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
    pub name: String,
    pub id: ChanId,
    state: sync::Mutex<ChannelState>,
}

// Ensure we're safe shipping Channels around, in particular on_connect is on
// another thread.
assert_impl_all!(Channel: Send);


fn get_field_type(id: ChanId) -> Option<BasicDbrType>
{
    caunion::get_field_type(unsafe { cadef::ca_field_type(id) })
}

fn get_element_count(id: ChanId) -> Option<usize>
{
    let count = unsafe { cadef::ca_element_count(id) };
    if count == 0 {
        // Treat this as disconnected
        None
    } else {
        Some(count as usize)
    }
}


// Called whenever the associated channel connection state changes.
extern fn on_connect(args: cadef::ca_connection_handler_args)
{
    let channel: &Channel = unsafe { voidp_to_ref(cadef::ca_puser(args.chid)) };
    let mut connected = false;
    let connection = match args.op {
        cadef::CA_OP_CONN_UP => {
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
        cadef::CA_OP_CONN_DOWN => {
            ChannelConnection::Disconnected
        },
        x => {
            println!("Unexpected connection state {}", x);
            ChannelConnection::Disconnected
        },
    };
    let mut state = channel.state.lock().unwrap();
    state.connection = connection;
    if connected {
        for waker in state.wakers.drain(..) {
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
            id: cadef::CHAN_ID_VOID,
            state: sync::Mutex::new(ChannelState {
                connection: ChannelConnection::Unconnected,
                wakers: Vec::new(),
            }),
        });

        let cpv = ffi::CString::new(pv).unwrap();
        let mut chan_id = cadef::CHAN_ID_VOID;
        let rc = unsafe {
            cadef::ca_create_channel(
                cpv.as_ptr(), on_connect, ref_to_voidp(channel.as_ref()),
                0, &mut chan_id as *mut ChanId) };
        assert!(rc == 1);

        channel.id = chan_id;
        channel
    }

    pub async fn wait_connect(&self) -> (BasicDbrType, usize)
    {
        ChannelWait::new(self).await
    }
}

// Code to ensure that the context is valid
static CA_CONTEXT_CREATE: sync::Once = sync::Once::new();
fn context_create()
{
    CA_CONTEXT_CREATE.call_once(|| {
        unsafe { cadef::ca_context_create(
            cadef::ca_preemptive_callback_select
                ::ca_enable_preemptive_callback) };
    });
}


impl Drop for Channel {
    fn drop(self: &mut Channel)
    {
        let rc = unsafe { cadef::ca_clear_channel(self.id) };
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
        if let ChannelConnection::Connected(field_type, field_count)
            = state.connection
        {
            task::Poll::Ready((field_type, field_count))
        } else {
            state.wakers.push(context.waker().clone());
            task::Poll::Pending
        }
    }
}

pub async fn connect(pv: &str) -> (Box<Channel>, BasicDbrType, usize)
{
    let channel = Channel::new(pv);
    let (datatype, count) = channel.wait_connect().await;
    (channel, datatype, count)
}
