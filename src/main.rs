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
        let result: Box<[String]> = CA::caget(pv).await;
        println!("Caget: {} => {:?}", pv, result);
        let result: Box<[f64]> = CA::caget(pv).await;
        println!("Caget: {} => {:?}", pv, result);
        let result: (f64, StatusSeverity, SystemTime) = CA::caget(pv).await;
        println!("Caget: {} => {:?} {}", pv, result, format_rfc3339(result.2));

        let result: (f64, CtrlLimits<f64>) = CA::caget(pv).await;
        println!("Caget: {} => {:?}", pv, result);
        let result: (f32, CtrlLimits<f32>) = CA::caget(pv).await;
        println!("Caget: {} => {:?}", pv, result);
        let result: (i32, CtrlLimits<i32>) = CA::caget(pv).await;
        println!("Caget: {} => {:?}", pv, result);
        let result: (i16, CtrlLimits<i16>) = CA::caget(pv).await;
        println!("Caget: {} => {:?}", pv, result);
        let result: (u8, CtrlLimits<u8>) = CA::caget(pv).await;
        println!("Caget: {} => {:?}", pv, result);
    });
}
