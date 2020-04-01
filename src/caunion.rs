// Definitions for union type

use libc::c_short;

use crate::db_access::dbr_type_code;
use crate::dbr::CaEnum;
use crate::channel;
use crate::caget::CaGetCore;


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

pub enum CaUnionVec {
    CaString(Vec<String>),
    CaEnum(Vec<CaEnum>),
    CaChar(Vec<u8>),
    CaShort(Vec<i16>),
    CaLong(Vec<i32>),
    CaFloat(Vec<f32>),
    CaDouble(Vec<f64>),
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

pub async fn caget_union(pv: &str) -> CaUnion
{
    let (channel, datatype, _count) = channel::connect(pv).await;
    match datatype {
        BasicDbrType::DbrString => CaUnion::CaString(
            CaGetCore::caget_core(&channel).await),
        BasicDbrType::DbrEnum => CaUnion::CaEnum(
            CaGetCore::caget_core(&channel).await),
        BasicDbrType::DbrChar => CaUnion::CaChar(
            CaGetCore::caget_core(&channel).await),
        BasicDbrType::DbrShort => CaUnion::CaShort(
            CaGetCore::caget_core(&channel).await),
        BasicDbrType::DbrLong => CaUnion::CaLong(
            CaGetCore::caget_core(&channel).await),
        BasicDbrType::DbrFloat => CaUnion::CaFloat(
            CaGetCore::caget_core(&channel).await),
        BasicDbrType::DbrDouble => CaUnion::CaDouble(
            CaGetCore::caget_core(&channel).await),
    }
}
