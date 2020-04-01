// Channel Access data formats
//
// These are all as defined in db_access.h in EPICS base

const MAX_STRING_SIZE: usize = 40;
const MAX_UNITS_SIZE: usize = 8;
const MAX_ENUM_STRING_SIZE: usize = 26;
const MAX_ENUM_STATES: usize = 16;


#[repr(C, packed)]
pub struct EpicsTimeStamp {
    pub secs: u32,
    pub nsec: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct StatusSeverity {
    pub status: i16,
    pub severity: i16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CtrlLimits<T: Copy + Send> {
    pub upper_disp_limit:     T,
    pub lower_disp_limit:     T,
    pub upper_alarm_limit:    T,
    pub upper_warning_limit:  T,
    pub lower_warning_limit:  T,
    pub lower_alarm_limit:    T,
    pub upper_ctrl_limit:     T,
    pub lower_ctrl_limit:     T,
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
    pub status_severity: StatusSeverity,
    pub raw_time: EpicsTimeStamp,
    pub value: EpicsString,
}


// Enum type

#[repr(C, packed)]
pub struct dbr_enum {
    pub value: u16,
}

#[repr(C, packed)]
pub struct dbr_time_enum {
    pub status_severity: StatusSeverity,
    pub raw_time: EpicsTimeStamp,
    _padding: u16,
    pub value: u16,
}

#[repr(C, packed)]
pub struct dbr_ctrl_enum {
    pub status_severity: StatusSeverity,
    pub enum_count: i16,
    pub strings: [[u8; MAX_ENUM_STRING_SIZE]; MAX_ENUM_STATES],
    pub value: u16,
}


// Integer types

#[repr(C, packed)]
pub struct dbr_char {
    pub value: u8,
}

#[repr(C, packed)]
pub struct dbr_time_char {
    pub status_severity: StatusSeverity,
    pub raw_time: EpicsTimeStamp,
    _padding0: u16,
    _padding1: u8,
    pub value: u8,
}

#[repr(C, packed)]
pub struct dbr_ctrl_char {
    pub status_severity: StatusSeverity,
    pub units: [u8; MAX_UNITS_SIZE],
    pub ctrl_limits: CtrlLimits<u8>,
    _padding1: u8,
    pub value: u8,
}


#[repr(C, packed)]
pub struct dbr_short {
    pub value: i16,
}

#[repr(C, packed)]
pub struct dbr_time_short {
    pub status_severity: StatusSeverity,
    pub raw_time: EpicsTimeStamp,
    _padding: u16,
    pub value: i16,
}

#[repr(C, packed)]
pub struct dbr_ctrl_short {
    pub status_severity: StatusSeverity,
    pub units: [u8; MAX_UNITS_SIZE],
    pub ctrl_limits: CtrlLimits<i16>,
    pub value: i16,
}


#[repr(C, packed)]
pub struct dbr_long {
    pub value: i32,
}

#[repr(C, packed)]
pub struct dbr_time_long {
    pub status_severity: StatusSeverity,
    pub raw_time: EpicsTimeStamp,
    pub value: i32,
}

#[repr(C, packed)]
pub struct dbr_ctrl_long {
    pub status_severity: StatusSeverity,
    pub units: [u8; MAX_UNITS_SIZE],
    pub ctrl_limits: CtrlLimits<i32>,
    pub value: i32,
}


// Floating point types

#[repr(C, packed)]
pub struct dbr_float {
    pub value: f32,
}

#[repr(C, packed)]
pub struct dbr_time_float {
    pub status_severity: StatusSeverity,
    pub raw_time: EpicsTimeStamp,
    pub value: f32,
}

#[repr(C, packed)]
pub struct dbr_ctrl_float {
    pub status_severity: StatusSeverity,
    pub precision: i16,
    _padding: i16,
    pub units: [u8; MAX_UNITS_SIZE],
    pub ctrl_limits: CtrlLimits<f32>,
    pub value: f32,
}


#[repr(C, packed)]
pub struct dbr_double {
    pub value: f64,
}

#[repr(C, packed)]
pub struct dbr_time_double {
    pub status_severity: StatusSeverity,
    pub raw_time: EpicsTimeStamp,
    _padding: i32,
    pub value: f64,
}

#[repr(C, packed)]
pub struct dbr_ctrl_double {
    pub status_severity: StatusSeverity,
    pub precision: i16,
    _padding: i16,
    pub units: [u8; MAX_UNITS_SIZE],
    pub ctrl_limits: CtrlLimits<f64>,
    pub value: f64,
}


pub mod dbr_type_code {
    pub const DBR_STRING: i16 = 0;
    pub const DBR_SHORT: i16 = 1;
    pub const DBR_FLOAT: i16 = 2;
    pub const DBR_ENUM: i16 = 3;
    pub const DBR_CHAR: i16 = 4;
    pub const DBR_LONG: i16 = 5;
    pub const DBR_DOUBLE: i16 = 6;
    pub const DBR_TIME_STRING: i16 = 14;
    pub const DBR_TIME_SHORT: i16 = 15;
    pub const DBR_TIME_FLOAT: i16 = 16;
    pub const DBR_TIME_ENUM: i16 = 17;
    pub const DBR_TIME_CHAR: i16 = 18;
    pub const DBR_TIME_LONG: i16 = 19;
    pub const DBR_TIME_DOUBLE: i16 = 20;
    pub const DBR_CTRL_SHORT: i16 = 29;
    pub const DBR_CTRL_FLOAT: i16 = 30;
    pub const DBR_CTRL_ENUM: i16 = 31;
    pub const DBR_CTRL_CHAR: i16 = 32;
    pub const DBR_CTRL_LONG: i16 = 33;
    pub const DBR_CTRL_DOUBLE: i16 = 34;
}
