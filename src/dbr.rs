// Channel Access data formats

#![allow(dead_code)]

use libc::c_short;

const MAX_STRING_SIZE: usize = 40;

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct EpicsTimeStamp {
    pub secs: u32,
    pub nsec: u32,
}

#[repr(C, packed)]
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

#[allow(unused_unsafe)]
unsafe fn c_array_to_vector<T: Copy>(array: &T, count: usize) -> Vec<T>
{
    let ptr = unsafe { array as *const T };
    let slice = unsafe { std::slice::from_raw_parts(ptr, count) };
    slice.to_vec()
}

#[repr(C, packed)]
pub struct dbr_string {
    value: EpicsString,
}

#[repr(C, packed)]
pub struct dbr_time_string {
    status_time: CaStatusTime,
    value: EpicsString,
}

// Integer types

#[repr(C, packed)]
pub struct dbr_char {
    value: u8,
}

#[repr(C, packed)]
pub struct dbr_time_char {
    status_time: CaStatusTime,
    _padding0: u16,
    _padding1: u8,
    value: u8,
}

#[repr(C, packed)]
pub struct dbr_short {
    value: i16,
}

#[repr(C, packed)]
pub struct dbr_time_short {
    status_time: CaStatusTime,
    _padding: u16,
    value: i16,
}

#[repr(C, packed)]
pub struct dbr_long {
    value: i32,
}

#[repr(C, packed)]
pub struct dbr_time_long {
    status_time: CaStatusTime,
    value: i32,
}


// Floating point types

#[repr(C, packed)]
pub struct dbr_float {
    value: f32,
}

#[repr(C, packed)]
pub struct dbr_time_float {
    status_time: CaStatusTime,
    value: f32,
}

#[repr(C, packed)]
pub struct dbr_double {
    value: f64,
}

#[repr(C, packed)]
pub struct dbr_time_double {
    status_time: CaStatusTime,
    _padding: i32,
    value: f64,
}


const DBR_STRING: c_short = 0;
const DBR_TIME_STRING: c_short = 14;
const DBR_SHORT:  c_short = 1;
const DBR_TIME_SHORT:  c_short = 15;
const DBR_FLOAT:  c_short = 2;
const DBR_TIME_FLOAT:  c_short = 16;
const DBR_ENUM:   c_short = 3;
const DBR_TIME_ENUM:   c_short = 17;
const DBR_CHAR:   c_short = 4;
const DBR_TIME_CHAR:   c_short = 18;
const DBR_LONG:   c_short = 5;
const DBR_TIME_LONG:   c_short = 19;
const DBR_DOUBLE: c_short = 6;
const DBR_TIME_DOUBLE: c_short = 20;



pub trait Dbr<R: Send, E: Send> {
    const DATATYPE: libc::c_long;
    fn get_value(&self) -> R;
    fn get_value_vec(&self, count: usize) -> Vec<R>;
    fn get_extra(&self) -> E;
}

pub trait DbrMap: Sized + Send {
    type ValueDbr: Dbr<Self, ()>;
    type TimeDbr: Dbr<Self, CaStatusTime>;
}

macro_rules! string_get_values {
    {} => {
        fn get_value(&self) -> String { from_epics_string(&self.value) }
        fn get_value_vec(&self, count: usize) -> Vec<String>
        {
            let slice = unsafe {
                std::slice::from_raw_parts(
                    &self.value as *const EpicsString, count) };
            slice.iter().map(from_epics_string).collect()
        }
    }
}

impl Dbr<String, ()> for dbr_string {
    const DATATYPE: libc::c_long = DBR_STRING as libc::c_long;

    string_get_values!{}

    fn get_extra(&self) -> () { () }
}

impl Dbr<String, CaStatusTime> for dbr_time_string {
    const DATATYPE: libc::c_long = DBR_TIME_STRING as libc::c_long;

    string_get_values!{}

    fn get_extra(&self) -> CaStatusTime { self.status_time }
}

impl DbrMap for String {
    type ValueDbr = dbr_string;
    type TimeDbr = dbr_time_string;
}


// -----------------------------------------------------------------------------
// Scalar types

macro_rules! scalar_dbr {
    { $type:ty,
        $value_const:expr, $value_dbr:ident, $time_const:expr, $time_dbr:ident
    } => {
        impl Dbr<$type, ()> for $value_dbr {
            const DATATYPE: libc::c_long = $value_const as libc::c_long;

            fn get_value(&self) -> $type { self.value }
            fn get_value_vec(&self, count: usize) -> Vec<$type>
            {
                unsafe { c_array_to_vector(&self.value, count) }
            }
            fn get_extra(&self) -> () { () }
        }

        impl Dbr<$type, CaStatusTime> for $time_dbr {
            const DATATYPE: libc::c_long = $time_const as libc::c_long;

            fn get_value(&self) -> $type { self.value }
            fn get_value_vec(&self, count: usize) -> Vec<$type>
            {
                unsafe { c_array_to_vector(&self.value, count) }
            }
            fn get_extra(&self) -> CaStatusTime { self.status_time }
        }

        impl DbrMap for $type {
            type ValueDbr = $value_dbr;
            type TimeDbr = $time_dbr;
        }
    }
}


scalar_dbr!{u8,  DBR_CHAR,   dbr_char,   DBR_TIME_CHAR,   dbr_time_char}
scalar_dbr!{i16, DBR_SHORT,  dbr_short,  DBR_TIME_SHORT,  dbr_time_short}
scalar_dbr!{i32, DBR_LONG,   dbr_long,   DBR_TIME_LONG,   dbr_time_long}
scalar_dbr!{f32, DBR_FLOAT,  dbr_float,  DBR_TIME_FLOAT,  dbr_time_float}
scalar_dbr!{f64, DBR_DOUBLE, dbr_double, DBR_TIME_DOUBLE, dbr_time_double}
