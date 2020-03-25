// Simple helper for library

use futures::executor::block_on;
use epics_ca::*;

fn main()
{
    block_on(async {
        let pv = "SR-DI-DCCT-01:SIGNAL";
        let result: f64 = CA::caget(pv).await;
        println!("Caget: {} => {}", pv, result);
        let result: i32 = CA::caget(pv).await;
        println!("Caget: {} => {}", pv, result);
        let result: String = CA::caget(pv).await;
        println!("Caget: {} => {}", pv, result);
        let result: Vec<f64> = CA::caget(pv).await;
        println!("Caget: {} => {:?}", pv, result);
        let result: (f64, CaStatusTime) = CA::caget(pv).await;
        println!("Caget: {} => {:?}", pv, result);
    });
}
