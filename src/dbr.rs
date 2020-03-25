// Channel Access data formats

#![allow(dead_code)]

use libc::c_short;

const MAX_STRING_SIZE: usize = 40;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EpicsTimeStamp {
    pub secs: u32,
    pub nsec: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CaStatusTime {
    pub status: i16,
    pub severity: i16,
    pub raw_time: EpicsTimeStamp,
}


// Declarations for the seven fundamental types: strings, char, short, long,
// float, double, enum, with raw, time+status, ctrl options.

// Strings

#[repr(transparent)]
pub struct EpicsString([u8; MAX_STRING_SIZE]);

fn from_epics_string(string: &EpicsString) -> String
{
    // Extract either a null terminated string or the entire string if not
    // null terminated.
    let string = string.0.split(|x| *x == 0).next().unwrap_or(&string.0);
    // Convert into internal UTF8 string, with replacement characters where
    // required.
    String::from_utf8_lossy(string).into_owned()

    // An alternative way of writing the above is this:
    //  let length = dbr.value.iter().position(|x| *x == 0).unwrap_or(40);
    //  String::from_utf8_lossy(&dbr.value[..length]).into_owned()
}

#[repr(C)]
pub struct dbr_string {
    value: EpicsString,
}

#[repr(C)]
pub struct dbr_time_string {
    status_time: CaStatusTime,
    value: EpicsString,
}

// Integer types

#[repr(C)]
pub struct dbr_char {
    value: u8,
}

#[repr(C)]
pub struct dbr_time_char {
    status_time: CaStatusTime,
    _padding0: u16,
    _padding1: u8,
    value: u8,
}

#[repr(C)]
pub struct dbr_short {
    value: i16,
}

#[repr(C)]
pub struct dbr_time_short {
    status_time: CaStatusTime,
    _padding: u16,
    value: i16,
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


// Floating point types

#[repr(C)]
pub struct dbr_float {
    value: f32,
}

#[repr(C)]
pub struct dbr_time_float {
    status_time: CaStatusTime,
    value: f32,
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


pub trait Adapter: Send + Sized {
    const DATATYPE: c_short;
    type DbrType;
    type DbrTimeType;
    fn get_value(dbr: &Self::DbrType) -> Self;
    fn get_value_vec(dbr: &Self::DbrType, count: usize) -> Vec<Self>;
    fn get_value_time(dbr: &Self::DbrTimeType) -> (Self, CaStatusTime);
}

impl Adapter for String {
    const DATATYPE: c_short = DBR_STRING;
    type DbrType = dbr_string;
    type DbrTimeType = dbr_time_string;

    fn get_value(dbr: &Self::DbrType) -> Self
    {
        from_epics_string(&dbr.value)
    }

    fn get_value_time(dbr: &Self::DbrTimeType) -> (Self, CaStatusTime)
    {
        (from_epics_string(&dbr.value), dbr.status_time)
    }

    fn get_value_vec(dbr: &Self::DbrType, count: usize) -> Vec<Self>
    {
        let slice = unsafe {
            std::slice::from_raw_parts(
                &dbr.value as *const EpicsString, count) };
        slice.iter().map(from_epics_string).collect()
    }
}

macro_rules! scalar_adapter {
    { $target:ident, $const:expr, $type:ty, $time_type:ty } => {
        impl Adapter for $target {
            const DATATYPE: c_short = $const;
            type DbrType = $type;
            type DbrTimeType = $time_type;

            fn get_value(dbr: &Self::DbrType) -> Self { dbr.value }

            fn get_value_time(dbr: &Self::DbrTimeType) -> (Self, CaStatusTime)
            {
                (dbr.value, dbr.status_time)
            }

            fn get_value_vec(dbr: &Self::DbrType, count: usize) -> Vec<Self> {
                let slice = unsafe {
                    std::slice::from_raw_parts(
                        &dbr.value as *const Self, count) };
                slice.to_vec()
            }

        }
    };
}

scalar_adapter!{u8,  DBR_CHAR,   dbr_char,   dbr_time_char}
scalar_adapter!{i16, DBR_SHORT,  dbr_short,  dbr_time_short}
scalar_adapter!{i32, DBR_LONG,   dbr_long,   dbr_time_long}
scalar_adapter!{f32, DBR_FLOAT,  dbr_float,  dbr_time_float}
scalar_adapter!{f64, DBR_DOUBLE, dbr_double, dbr_time_double}
