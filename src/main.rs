// Simple helper for library

use futures::executor::block_on;
use epics_ca::*;

fn main()
{
    let pv = "SR-DI-DCCT-01:SIGNAL";
    let result: f64 = block_on(CA::caget(pv));
    println!("Caget: {} => {}", pv, result);
    let result: i32 = block_on(CA::caget(pv));
    println!("Caget: {} => {}", pv, result);
    let result: String = block_on(CA::caget(pv));
    println!("Caget: {} => {}", pv, result);
    let result: Vec<f64> = block_on(CA::caget_vec(pv));
    println!("Caget: {} => {:?}", pv, result);
}
