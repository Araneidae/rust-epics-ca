// Simple helper for library

mod my_future;

use futures::executor::block_on;
use epics_ca;

fn main()
{
    let channel1 = epics_ca::Channel::new("SR-DI-DCCT-01:SIGNAL");

    block_on(channel1.wait_connect());

    println!("Channel1: {:?}", channel1);
}
