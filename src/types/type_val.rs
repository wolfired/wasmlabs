use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;
use std::io::Seek;
use std::io::SeekFrom;

use crate::scan::Scan;

use super::type_num::NumType;
use super::type_ref::RefType;
use super::type_vec::VecType;

#[derive(Copy, Debug, Clone)]
pub enum ValType {
    NumType(NumType),
    VecType(VecType),
    RefType(RefType),
}

impl Display for ValType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NumType(n) => write!(f, "{}", n),
            Self::VecType(v) => write!(f, "{}", v),
            Self::RefType(r) => write!(f, "{}", r),
        }
    }
}

impl From<ValType> for u8 {
    fn from(value: ValType) -> Self {
        match value {
            ValType::NumType(n) => n.into(),
            ValType::VecType(v) => v.into(),
            ValType::RefType(r) => r.into(),
        }
    }
}

impl Scan for ValType {
    fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        if let Ok(n) = NumType::scan(cursor) {
            return Ok(ValType::NumType(n));
        } else {
            cursor.seek(SeekFrom::Current(-1))?;
        }

        if let Ok(v) = VecType::scan(cursor) {
            return Ok(ValType::VecType(v));
        } else {
            cursor.seek(SeekFrom::Current(-1))?;
        }

        if let Ok(r) = RefType::scan(cursor) {
            return Ok(ValType::RefType(r));
        }

        Err("undefine valtype".into())
    }
}
