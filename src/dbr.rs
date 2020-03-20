// Channel Access data formats

#![allow(dead_code)]

use libc::{c_char, c_double};

pub const MAX_STRING_SIZE: usize = 40;

#[repr(C)]
pub struct dbr_string {
    pub value: [c_char; MAX_STRING_SIZE],
}

#[repr(C)]
pub struct dbr_double {
    pub value: c_double,
}
