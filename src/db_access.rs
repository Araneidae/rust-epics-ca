// Channel Access data formats
//
// These are all as defined in db_access.h in EPICS base

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
pub struct EpicsString(pub [u8; MAX_STRING_SIZE]);

#[repr(C, packed)]
pub struct dbr_string {
    pub value: EpicsString,
}

#[repr(C, packed)]
pub struct dbr_time_string {
    pub status_time: CaStatusTime,
    pub value: EpicsString,
}

// Integer types

#[repr(C, packed)]
pub struct dbr_char {
    pub value: u8,
}

#[repr(C, packed)]
pub struct dbr_time_char {
    pub status_time: CaStatusTime,
    _padding0: u16,
    _padding1: u8,
    pub value: u8,
}

#[repr(C, packed)]
pub struct dbr_short {
    pub value: i16,
}

#[repr(C, packed)]
pub struct dbr_time_short {
    pub status_time: CaStatusTime,
    _padding: u16,
    pub value: i16,
}

#[repr(C, packed)]
pub struct dbr_long {
    pub value: i32,
}

#[repr(C, packed)]
pub struct dbr_time_long {
    pub status_time: CaStatusTime,
    pub value: i32,
}


// Floating point types

#[repr(C, packed)]
pub struct dbr_float {
    pub value: f32,
}

#[repr(C, packed)]
pub struct dbr_time_float {
    pub status_time: CaStatusTime,
    pub value: f32,
}

#[repr(C, packed)]
pub struct dbr_double {
    pub value: f64,
}

#[repr(C, packed)]
pub struct dbr_time_double {
    pub status_time: CaStatusTime,
    _padding: i32,
    pub value: f64,
}


#[allow(non_camel_case_types)]
pub enum DbrTypeCode {
    DBR_STRING = 0,
    DBR_SHORT = 1,
    DBR_FLOAT = 2,
    DBR_ENUM = 3,
    DBR_CHAR = 4,
    DBR_LONG = 5,
    DBR_DOUBLE = 6,
    DBR_TIME_STRING = 14,
    DBR_TIME_SHORT = 15,
    DBR_TIME_FLOAT = 16,
    DBR_TIME_ENUM = 17,
    DBR_TIME_CHAR = 18,
    DBR_TIME_LONG = 19,
    DBR_TIME_DOUBLE = 20,
}
