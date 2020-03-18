// Simple helper for library

use epics_ca;

fn main()
{
    let _channel1 = epics_ca::Channel::new("SR-DI-DCCT-01:SIGNAL");
    let _channel2 = epics_ca::Channel::new("SR-DI-DCCT-01:SIGNAL");
    std::thread::sleep(std::time::Duration::from_secs(1));
}
