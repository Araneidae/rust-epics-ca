// Definitions for union type

use libc::c_short;
use std::time::SystemTime;
use async_trait::async_trait;

use crate::db_access::dbr_type_code;
use crate::db_access::StatusSeverity;
use crate::dbr::{CaEnum, FixedCtrl, FloatCtrl};
use crate::channel;
use crate::caget::{CaGetCore, CaCtrl, CA};


#[derive(Clone, Copy, Debug)]
pub enum BasicDbrType {
    DbrString,
    DbrEnum,
    DbrChar,
    DbrShort,
    DbrLong,
    DbrFloat,
    DbrDouble,
}

#[derive(Debug, Clone)]
pub enum CaUnion {
    CaString(String),
    CaEnum(CaEnum),
    CaChar(u8),
    CaShort(i16),
    CaLong(i32),
    CaFloat(f32),
    CaDouble(f64),
}

#[derive(Debug, Clone)]
pub enum CaUnionVec {
    CaString(Vec<String>),
    CaEnum(Vec<CaEnum>),
    CaChar(Vec<u8>),
    CaShort(Vec<i16>),
    CaLong(Vec<i32>),
    CaFloat(Vec<f32>),
    CaDouble(Vec<f64>),
}

#[derive(Debug)]
pub enum CaUnionCtrl {
    CaString(String, SystemTime),
    CaEnum(CaEnum, Vec<String>),
    CaChar(u8, FixedCtrl<u8>),
    CaShort(i16, FixedCtrl<i16>),
    CaLong(i32, FixedCtrl<i32>),
    CaFloat(f32, FloatCtrl<f32>),
    CaDouble(f64, FloatCtrl<f64>),
}

#[derive(Debug)]
pub enum CaUnionCtrlVec {
    CaString(Vec<String>, SystemTime),
    CaEnum(Vec<CaEnum>, Vec<String>),
    CaChar(Vec<u8>, FixedCtrl<u8>),
    CaShort(Vec<i16>, FixedCtrl<i16>),
    CaLong(Vec<i32>, FixedCtrl<i32>),
    CaFloat(Vec<f32>, FloatCtrl<f32>),
    CaDouble(Vec<f64>, FloatCtrl<f64>),
}


pub fn get_field_type(field_type: c_short) -> Option<BasicDbrType>
{
    match field_type {
        dbr_type_code::DBR_STRING => Some(BasicDbrType::DbrString),
        dbr_type_code::DBR_ENUM   => Some(BasicDbrType::DbrEnum),
        dbr_type_code::DBR_CHAR   => Some(BasicDbrType::DbrChar),
        dbr_type_code::DBR_SHORT  => Some(BasicDbrType::DbrShort),
        dbr_type_code::DBR_LONG   => Some(BasicDbrType::DbrLong),
        dbr_type_code::DBR_FLOAT  => Some(BasicDbrType::DbrFloat),
        dbr_type_code::DBR_DOUBLE => Some(BasicDbrType::DbrDouble),

        // None of the above.  Probably unexpectedly disconnected
        _ => None,
    }
}


macro_rules! map_caget_over_union {
    { $pv:expr, $action:ident } => {
        let (channel, datatype, _count) = channel::connect($pv).await;
        match datatype {
            BasicDbrType::DbrString => $action!(channel, CaString),
            BasicDbrType::DbrEnum   => $action!(channel, CaEnum),
            BasicDbrType::DbrChar   => $action!(channel, CaChar),
            BasicDbrType::DbrShort  => $action!(channel, CaShort),
            BasicDbrType::DbrLong   => $action!(channel, CaLong),
            BasicDbrType::DbrFloat  => $action!(channel, CaFloat),
            BasicDbrType::DbrDouble => $action!(channel, CaDouble),
        }
    }
}


#[async_trait(?Send)]
impl CA for CaUnion {
    async fn caget(pv: &str) -> Self {
        macro_rules! do_caget {
            ( $channel:expr, $result:ident ) => {
                CaUnion::$result(CaGetCore::caget_core(&$channel).await)
            }
        }

        map_caget_over_union!{pv, do_caget}
    }
}

#[async_trait(?Send)]
impl CA for (CaUnion, StatusSeverity, SystemTime) {
    async fn caget(pv: &str) -> Self {
        macro_rules! do_caget {
            ( $channel:expr, $result:ident ) => {
                {
                    let (v, s, t) = CaGetCore::caget_core(&$channel).await;
                    (CaUnion::$result(v), s, t)
                }
            }
        }

        map_caget_over_union!{pv, do_caget}
    }
}

#[async_trait(?Send)]
impl CA for CaUnionVec {
    async fn caget(pv: &str) -> Self {
        macro_rules! do_caget {
            ( $channel:expr, $result:ident ) => {
                CaUnionVec::$result(CaGetCore::caget_core(&$channel).await)
            }
        }

        map_caget_over_union!{pv, do_caget}
    }
}

#[async_trait(?Send)]
impl CA for (CaUnionVec, StatusSeverity, SystemTime) {
    async fn caget(pv: &str) -> Self {
        macro_rules! do_caget {
            ( $channel:expr, $result:ident ) => {
                {
                    let (v, s, t) = CaGetCore::caget_core(&$channel).await;
                    (CaUnionVec::$result(v), s, t)
                }
            }
        }

        map_caget_over_union!{pv, do_caget}
    }
}

#[async_trait(?Send)]
impl CA for (CaUnionCtrl, StatusSeverity) {
    async fn caget(pv: &str) -> Self {
        macro_rules! do_caget {
            ( $channel:expr, $result:ident ) => {
                {
                    let (v, s, CaCtrl(c)) =
                        CaGetCore::caget_core(&$channel).await;
                    (CaUnionCtrl::$result(v, c), s)
                }
            }
        }

        map_caget_over_union!{pv, do_caget}
    }
}

#[async_trait(?Send)]
impl CA for (CaUnionCtrlVec, StatusSeverity) {
    async fn caget(pv: &str) -> Self {
        macro_rules! do_caget {
            ( $channel:expr, $result:ident ) => {
                {
                    let (v, s, CaCtrl(c)) =
                        CaGetCore::caget_core(&$channel).await;
                    (CaUnionCtrlVec::$result(v, c), s)
                }
            }
        }

        map_caget_over_union!{pv, do_caget}
    }
}
