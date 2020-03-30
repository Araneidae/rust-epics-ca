// Channel Access data formats
//
// These are all as defined in db_access.h in EPICS base

use std::time::*;

use crate::db_access;
use db_access::*;


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
unsafe fn c_array_to_vector<T: Copy>(array: &T, count: usize) -> Box<[T]>
{
    let ptr = unsafe { array as *const T };
    let slice = unsafe { std::slice::from_raw_parts(ptr, count) };
    slice.into()
}


// The EPICS Epoch is based on 1st January 1990, and we want to convert all our
// timestamps to the UNIX_EPOCH of 1st January 1970 to create SystemTime
// instances.
const EPICS_EPOCH_SECS: u64 = 631152000;    // Seconds from 1970 to 1990

fn from_raw_stamp(epics_time: &EpicsTimeStamp) -> SystemTime
{
    let duration = Duration::new(epics_time.secs as u64, epics_time.nsec);
    let epics_epoch = Duration::new(EPICS_EPOCH_SECS, 0);
    UNIX_EPOCH.checked_add(epics_epoch).unwrap().checked_add(duration).unwrap()
}


pub trait Dbr<R: Send, E: Send> {
    const DATATYPE: DbrTypeCode;
    fn get_value(&self) -> R;
    fn get_value_vec(&self, count: usize) -> Box<[R]>;
    fn get_extra(&self) -> E;
}

pub trait DbrMap: Sized + Send {
    type ValueDbr: Dbr<Self, ()>;
    type TimeDbr: Dbr<Self, (StatusSeverity, SystemTime)>;
}

macro_rules! string_get_values {
    {} => {
        fn get_value(&self) -> String { from_epics_string(&self.value) }
        fn get_value_vec(&self, count: usize) -> Box<[String]>
        {
            let slice = unsafe {
                std::slice::from_raw_parts(
                    &self.value as *const EpicsString, count) };
            slice.iter().map(from_epics_string).collect()
        }
    }
}

impl Dbr<String, ()> for dbr_string {
    const DATATYPE: DbrTypeCode = DbrTypeCode::DBR_STRING;

    string_get_values!{}

    fn get_extra(&self) -> () { () }
}

impl Dbr<String, (StatusSeverity, SystemTime)> for dbr_time_string {
    const DATATYPE: DbrTypeCode = DbrTypeCode::DBR_TIME_STRING;

    string_get_values!{}

    fn get_extra(&self) -> (StatusSeverity, SystemTime) {
        (self.status_severity, from_raw_stamp(&self.raw_time))
    }
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
            const DATATYPE: DbrTypeCode = $value_const;

            fn get_value(&self) -> $type { self.value }
            fn get_value_vec(&self, count: usize) -> Box<[$type]>
            {
                unsafe { c_array_to_vector(&self.value, count) }
            }
            fn get_extra(&self) -> () { () }
        }

        impl Dbr<$type, (StatusSeverity, SystemTime)> for $time_dbr {
            const DATATYPE: DbrTypeCode = $time_const;

            fn get_value(&self) -> $type { self.value }
            fn get_value_vec(&self, count: usize) -> Box<[$type]>
            {
                unsafe { c_array_to_vector(&self.value, count) }
            }
            fn get_extra(&self) -> (StatusSeverity, SystemTime) {
                (self.status_severity, from_raw_stamp(&self.raw_time))
            }
        }

        impl DbrMap for $type {
            type ValueDbr = $value_dbr;
            type TimeDbr = $time_dbr;
        }
    }
}


use DbrTypeCode::*;
scalar_dbr!{u8,  DBR_CHAR,   dbr_char,   DBR_TIME_CHAR,   dbr_time_char}
scalar_dbr!{i16, DBR_SHORT,  dbr_short,  DBR_TIME_SHORT,  dbr_time_short}
scalar_dbr!{i32, DBR_LONG,   dbr_long,   DBR_TIME_LONG,   dbr_time_long}
scalar_dbr!{f32, DBR_FLOAT,  dbr_float,  DBR_TIME_FLOAT,  dbr_time_float}
scalar_dbr!{f64, DBR_DOUBLE, dbr_double, DBR_TIME_DOUBLE, dbr_time_double}
