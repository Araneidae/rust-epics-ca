// Definitions for union type

use libc::c_short;

use crate::db_access;
use crate::db_access::dbr_type_code;
use crate::dbr;
use crate::channel;


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
    CaEnum(dbr::CaEnum),
    CaChar(u8),
    CaShort(i16),
    CaLong(i32),
    CaFloat(f32),
    CaDouble(f64),
}

pub enum CaUnionVec {
    CaString(Box<[String]>),
    CaEnum(Box<[dbr::CaEnum]>),
    CaChar(Box<[u8]>),
    CaShort(Box<[i16]>),
    CaLong(Box<[i32]>),
    CaFloat(Box<[f32]>),
    CaDouble(Box<[f64]>),
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

    println!("Connected channel: {:?}", datatype);
    match datatype {
        BasicDbrType::DbrString => CaUnion::CaString(
            crate::caget_core::<db_access::dbr_string, _>(channel).await.0),
        BasicDbrType::DbrEnum => CaUnion::CaEnum(
            crate::caget_core::<db_access::dbr_enum, _>(channel).await.0),
        BasicDbrType::DbrChar => CaUnion::CaChar(
            crate::caget_core::<db_access::dbr_char, _>(channel).await.0),
        BasicDbrType::DbrShort => CaUnion::CaShort(
            crate::caget_core::<db_access::dbr_short, _>(channel).await.0),
        BasicDbrType::DbrLong => CaUnion::CaLong(
            crate::caget_core::<db_access::dbr_long, _>(channel).await.0),
        BasicDbrType::DbrFloat => CaUnion::CaFloat(
            crate::caget_core::<db_access::dbr_float, _>(channel).await.0),
        BasicDbrType::DbrDouble => CaUnion::CaDouble(
            crate::caget_core::<db_access::dbr_double, _>(channel).await.0),
    }
}
