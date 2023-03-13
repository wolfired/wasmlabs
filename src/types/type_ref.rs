use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;
use std::io::Read;

use crate::scan::Scan;

#[derive(Copy, Debug, Clone)]
pub enum RefType {
    FuncRef = 0x70,
    ExternRef = 0x6F,
}

impl Display for RefType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FuncRef => write!(f, "\"funcref\""),
            Self::ExternRef => write!(f, "\"externref\""),
        }
    }
}

impl From<RefType> for u8 {
    fn from(value: RefType) -> Self {
        match value {
            RefType::FuncRef => 0x70,
            RefType::ExternRef => 0x6F,
        }
    }
}

impl Scan for RefType {
    fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        let mut byte = [0; 1];
        cursor.read_exact(&mut byte)?;
        match byte {
            [0x70] => Ok(Self::FuncRef),
            [0x6F] => Ok(Self::ExternRef),
            _ => Err("undefine reftype".into()),
        }
    }
}
