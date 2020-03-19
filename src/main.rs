// Simple helper for library

use futures::executor::block_on;
use epics_ca;

fn main()
{
    let pv = "SR-DI-DCCT-01:SIGNAL";
    let result = block_on(epics_ca::caget(pv));
    println!("{} => {}", pv, result);
}
