// Channel Access data formats
//
// These are all as defined in db_access.h in EPICS base

const MAX_STRING_SIZE: usize = 40;
const MAX_UNITS_SIZE: usize = 8;

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
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
    pub units:                [u8; MAX_UNITS_SIZE],
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
    pub status: StatusSeverity,
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
    pub ctrl_limits: CtrlLimits<f64>,
    pub value: f64,
}


#[allow(non_camel_case_types)]
#[derive(Debug)]
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
    DBR_CTRL_SHORT = 29,
    DBR_CTRL_FLOAT = 30,
    DBR_CTRL_ENUM = 31,
    DBR_CTRL_CHAR = 32,
    DBR_CTRL_LONG = 33,
    DBR_CTRL_DOUBLE = 34,
}
