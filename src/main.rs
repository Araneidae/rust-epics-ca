// Simple helper for library

use epics_ca;

fn main()
{
    epics_ca::context_create();
    let _channel = epics_ca::Channel::new("SR-DI-DCCT-01:SIGNAL");
    std::thread::sleep(std::time::Duration::from_secs(1));
}
