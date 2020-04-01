// Simple helper for library

use futures::executor::block_on;
use epics_ca::*;
use humantime::format_rfc3339;

fn main()
{
    block_on(async {
        let pv = "SR-DI-DCCT-01:SIGNAL";

        let result: f64 = CA::caget(pv).await;
        println!("Caget: {} => {}", pv, result);
        let result: i32 = CA::caget(pv).await;
        println!("Caget: {} => {}", pv, result);
        let result: String = CA::caget(pv).await;
        println!("Caget: {} => {:?}", pv, result);
        let result: Vec<String> = CA::caget(pv).await;
        println!("Caget: {} => {:?}", pv, result);
        let result: Vec<f64> = CA::caget(pv).await;
        println!("Caget: {} => {:?}", pv, result);
        let result: CaEnum = CA::caget(pv).await;
        println!("Caget: {} => {:?}", pv, result);

        let (result, status, time): (f64, _, SystemTime) = CA::caget(pv).await;
        println!("Caget: {} => {}\n {:#?} {}", pv,
            result, status, format_rfc3339(time));

        let result: (f64, _, CaCtrl<_>) = CA::caget(pv).await;
        println!("Caget: {} => {:#?}", pv, result);
        let result: (f32, _, CaCtrl<_>) = CA::caget(pv).await;
        println!("Caget: {} => {:#?}", pv, result);
        let result: (i32, _, CaCtrl<_>) = CA::caget(pv).await;
        println!("Caget: {} => {:#?}", pv, result);
        let result: (i16, _, CaCtrl<_>) = CA::caget(pv).await;
        println!("Caget: {} => {:#?}", pv, result);
        let result: (u8, _, CaCtrl<_>) = CA::caget(pv).await;
        println!("Caget: {} => {:#?}", pv, result);
        let result: (CaEnum, _, CaCtrl<_>) = CA::caget(pv).await;
        println!("Caget: {} => {:#?}", pv, result);

        let result: CaUnion = CA::caget(pv).await;
        println!("Caget: {} => {:#?}", pv, result);
    });
}

// Try: unions, and async gather
