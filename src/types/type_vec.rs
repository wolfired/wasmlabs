use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;
use std::io::Read;

use crate::scan::Scan;

#[derive(Copy, Debug, Clone)]
pub enum VecType {
    V128 = 0x7B,
}

impl Display for VecType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::V128 => write!(f, "\"v128\""),
        }
    }
}

impl From<VecType> for u8 {
    fn from(value: VecType) -> Self {
        match value {
            VecType::V128 => 0x7B,
        }
    }
}

impl Scan for VecType {
    fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        let mut byte = [0; 1];
        cursor.read_exact(&mut byte)?;
        match byte {
            [0x7B] => Ok(Self::V128),
            _ => Err("undefine vectype".into()),
        }
    }
}
