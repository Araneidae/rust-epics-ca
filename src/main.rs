// Simple helper for library

use epics_ca;

fn main()
{
    epics_ca::safe_context_create();
    epics_ca::create_channel("SR-DI-DCCT-01:SIGNAL");
    unsafe { epics_ca::ca_pend_event(1.0) };
}
