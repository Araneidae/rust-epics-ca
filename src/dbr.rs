// Channel Access data formats
//
// These are all as defined in db_access.h in EPICS base

use std::time::*;

use crate::db_access;
use db_access::*;
use db_access::dbr_type_code::*;


fn from_epics_string(string: &[u8]) -> String
{
    // Extract either a null terminated string or the entire string if not
    // null terminated.
    let string = string.split(|x| *x == 0).next().unwrap_or(&string);
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


#[allow(dead_code)]
fn get_raw_bytes<T: Sized>(value: &T) -> &[u8]
{
    let length = std::mem::size_of::<T>();
    let bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(value as *const _ as *const u8, length) };
    bytes
//     println!("dump: {:02x?}", bytes);
}


// -----------------------------------------------------------------------------
// Traits defining interface to the dbrs

pub trait Dbr {
    const DATATYPE: i16;
    type ResultType: Send;
    type ExtraType: Send;
    fn get_value(&self) -> Self::ResultType;
    fn get_value_vec(&self, count: usize) -> Vec<Self::ResultType>;
    fn get_extra(&self) -> Self::ExtraType;
}

pub trait DbrMap: Sized + Send {
    type ValueDbr: Dbr<ResultType=Self, ExtraType=()>;
    type TimeDbr: Dbr<ResultType=Self, ExtraType=(StatusSeverity, SystemTime)>;
    type CtrlType: Send;
    type CtrlDbr: Dbr<
        ResultType=Self, ExtraType=(StatusSeverity, Self::CtrlType)>;
}


// -----------------------------------------------------------------------------
// String types

macro_rules! string_get_values {
    {} => {
        fn get_value(&self) -> Self::ResultType {
            from_epics_string(&self.value.0)
        }
        fn get_value_vec(&self, count: usize) -> Vec<Self::ResultType>
        {
            let slice = unsafe {
                std::slice::from_raw_parts(
                    &self.value as *const EpicsString, count) };
            slice.iter().map(|s| from_epics_string(&s.0)).collect()
        }
    }
}

impl Dbr for dbr_string {
    const DATATYPE: i16 = dbr_type_code::DBR_STRING;
    type ResultType = String;
    type ExtraType = ();

    string_get_values!{}

    fn get_extra(&self) -> Self::ExtraType { () }
}

impl Dbr for dbr_time_string {
    const DATATYPE: i16 = dbr_type_code::DBR_TIME_STRING;
    type ResultType = String;
    type ExtraType = (StatusSeverity, SystemTime);

    string_get_values!{}

    fn get_extra(&self) -> Self::ExtraType {
        (self.status_severity, from_raw_stamp(&self.raw_time))
    }
}


impl DbrMap for String {
    type ValueDbr = dbr_string;
    type TimeDbr = dbr_time_string;
    type CtrlType = SystemTime;
    type CtrlDbr = dbr_time_string;
}


// -----------------------------------------------------------------------------
// Enum type

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct CaEnum(pub u16);

macro_rules! enum_get_values {
    {} => {
        fn get_value(&self) -> Self::ResultType { CaEnum(self.value) }
        fn get_value_vec(&self, count: usize) -> Vec<Self::ResultType>
        {
            let values = unsafe { c_array_to_vector(&self.value, count) };
            values.iter().map(|&x| CaEnum(x)).collect()
        }
    }
}

impl Dbr for dbr_enum {
    const DATATYPE: i16 = dbr_type_code::DBR_ENUM;
    type ResultType = CaEnum;
    type ExtraType = ();

    enum_get_values!{}

    fn get_extra(&self) -> Self::ExtraType { () }
}

impl Dbr for dbr_time_enum {
    const DATATYPE: i16 = dbr_type_code::DBR_TIME_ENUM;
    type ResultType = CaEnum;
    type ExtraType = (StatusSeverity, SystemTime);

    enum_get_values!{}

    fn get_extra(&self) -> Self::ExtraType {
        (self.status_severity, from_raw_stamp(&self.raw_time))
    }
}

impl Dbr for dbr_ctrl_enum {
    const DATATYPE: i16 = dbr_type_code::DBR_CTRL_ENUM;
    type ResultType = CaEnum;
    type ExtraType = (StatusSeverity, Vec<String>);

    enum_get_values!{}

    fn get_extra(&self) -> Self::ExtraType {
        let enums = self.strings
            .iter().take(self.enum_count as usize)
            .map(|s| from_epics_string(s)).collect();
        (self.status_severity, enums)
    }
}

impl DbrMap for CaEnum {
    type ValueDbr = dbr_enum;
    type TimeDbr = dbr_time_enum;
    type CtrlType = Vec<String>;
    type CtrlDbr = dbr_ctrl_enum;
}


// -----------------------------------------------------------------------------
// Scalar types

#[derive(Debug)]
pub struct FixedCtrl<T: Copy + Send> {
    pub units: String,
    pub limits: CtrlLimits<T>,
}

#[derive(Debug)]
pub struct FloatCtrl<T: Copy + Send> {
    pub units: String,
    pub precision: i16,
    pub limits: CtrlLimits<T>,
}


macro_rules! scalar_get_values {
    {} => {
        fn get_value(&self) -> Self::ResultType { self.value }
        fn get_value_vec(&self, count: usize) -> Vec<Self::ResultType>
        {
            unsafe { c_array_to_vector(&self.value, count) }
        }
    }
}

macro_rules! fixed_limits {
    ($self:ident) => {
        FixedCtrl {
            units: from_epics_string(&$self.units),
            limits: $self.ctrl_limits,
        }
    }
}
macro_rules! float_limits {
    ($self:ident) => {
        FloatCtrl {
            units: from_epics_string(&$self.units),
            precision: $self.precision,
            limits: $self.ctrl_limits,
        }
    }
}

macro_rules! scalar_dbr {
    { $type:ty,
        $value_const:expr, $value_dbr:ident,
        $time_const:expr, $time_dbr:ident,
        $ctrl_const:expr, $ctrl_dbr:ident,
        $ctrl_type:tt, $ctrl_eval:ident
    } => {
        impl Dbr for $value_dbr {
            const DATATYPE: i16 = $value_const;
            type ResultType = $type;
            type ExtraType = ();

            scalar_get_values!{}

            fn get_extra(&self) -> Self::ExtraType { () }
        }

        impl Dbr for $time_dbr {
            const DATATYPE: i16 = $time_const;
            type ResultType = $type;
            type ExtraType = (StatusSeverity, SystemTime);

            scalar_get_values!{}

            fn get_extra(&self) -> Self::ExtraType {
                (self.status_severity, from_raw_stamp(&self.raw_time))
            }
        }

        impl Dbr for $ctrl_dbr {
            const DATATYPE: i16 = $ctrl_const;
            type ResultType = $type;
            type ExtraType = (StatusSeverity, $ctrl_type<$type>);

            scalar_get_values!{}

            fn get_extra(&self) -> Self::ExtraType {
                (self.status_severity, $ctrl_eval!(self))
            }
        }

        impl DbrMap for $type {
            type ValueDbr = $value_dbr;
            type TimeDbr = $time_dbr;
            type CtrlType = $ctrl_type<$type>;
            type CtrlDbr = $ctrl_dbr;
        }
    }
}


scalar_dbr!{u8,
    DBR_CHAR,           dbr_char,
    DBR_TIME_CHAR,      dbr_time_char,
    DBR_CTRL_CHAR,      dbr_ctrl_char,      FixedCtrl, fixed_limits }
scalar_dbr!{i16,
    DBR_SHORT,          dbr_short,
    DBR_TIME_SHORT,     dbr_time_short,
    DBR_CTRL_SHORT,     dbr_ctrl_short,     FixedCtrl, fixed_limits }
scalar_dbr!{i32,
    DBR_LONG,           dbr_long,
    DBR_TIME_LONG,      dbr_time_long,
    DBR_CTRL_LONG,      dbr_ctrl_long,      FixedCtrl, fixed_limits }
scalar_dbr!{f32,
    DBR_FLOAT,          dbr_float,
    DBR_TIME_FLOAT,     dbr_time_float,
    DBR_CTRL_FLOAT,     dbr_ctrl_float,     FloatCtrl, float_limits }
scalar_dbr!{f64,
    DBR_DOUBLE,         dbr_double,
    DBR_TIME_DOUBLE,    dbr_time_double,
    DBR_CTRL_DOUBLE,    dbr_ctrl_double,    FloatCtrl, float_limits }
