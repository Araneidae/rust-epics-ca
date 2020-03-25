// Channel Access data formats

#![allow(dead_code)]

use libc::{c_short};

const MAX_STRING_SIZE: usize = 40;

#[repr(C)]
struct EpicsTimeStamp {
    secs: u32,
    nsec: u32,
}

#[repr(C)]
struct CaStatusTime {
    status: i16,
    severity: i16,
    raw_time: EpicsTimeStamp,
}

#[repr(C)]
pub struct dbr_string {
    value: [i8; MAX_STRING_SIZE],
}

#[repr(C)]
pub struct dbr_time_string {
    status_time: CaStatusTime,
    value: [i8; MAX_STRING_SIZE],
}

#[repr(C)]
pub struct dbr_long {
    value: i32,
}

#[repr(C)]
pub struct dbr_time_long {
    status_time: CaStatusTime,
    value: i32,
}

#[repr(C)]
pub struct dbr_double {
    value: f64,
}

#[repr(C)]
pub struct dbr_time_double {
    status_time: CaStatusTime,
    _padding: i32,
    value: f64,
}


const DBR_STRING: c_short = 0;
const DBR_SHORT:  c_short = 1;
const DBR_FLOAT:  c_short = 2;
const DBR_ENUM:   c_short = 3;
const DBR_CHAR:   c_short = 4;
const DBR_LONG:   c_short = 5;
const DBR_DOUBLE: c_short = 6;


pub trait Adapter: Send + Copy {
    const DATATYPE: c_short;
    type DbrType;
    fn get_value(dbr: &Self::DbrType) -> &Self;
}

// impl Adapter for String {
//     const DATATYPE: c_short = DBR_STRING;
//     type DbrType = dbr_string;
//     fn get_value(dbr: &Self::DbrType) -> &Self { &dbr.value }
// }

// impl Adapter for i16 {
//     const DATATYPE: c_short = DBR_SHORT;
//     type DbrType = dbr_short;
//     fn get_value(dbr: &Self::DbrType) -> &Self { &dbr.value }
// }
// 
// impl Adapter for f32 {
//     const DATATYPE: c_short = DBR_FLOAT;
//     type DbrType = dbr_float;
//     fn get_value(dbr: &Self::DbrType) -> &Self { &dbr.value }
// }

impl Adapter for i32 {
    const DATATYPE: c_short = DBR_LONG;
    type DbrType = dbr_long;
    fn get_value(dbr: &Self::DbrType) -> &Self { &dbr.value }
}

impl Adapter for f64 {
    const DATATYPE: c_short = DBR_DOUBLE;
    type DbrType = dbr_double;
    fn get_value(dbr: &Self::DbrType) -> &Self { &dbr.value }
}
