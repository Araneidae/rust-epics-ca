// Simple helper for library

use epics_ca;

fn main()
{
    epics_ca::context_create();
    let _channel = epics_ca::Channel::new("SR-DI-DCCT-01:SIGNAL");
    epics_ca::safe_pend_event(1.0);

    // thread::sleep(std::time::Duration(1, 0));
}
