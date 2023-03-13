use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;
use std::io::Read;

use crate::scan::Scan;

#[derive(Copy, Debug, Clone)]
pub enum NumType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C,
}

impl Display for NumType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I32 => write!(f, "\"i32\""),
            Self::I64 => write!(f, "\"i64\""),
            Self::F32 => write!(f, "\"f32\""),
            Self::F64 => write!(f, "\"f64\""),
        }
    }
}

impl From<NumType> for u8 {
    fn from(value: NumType) -> Self {
        match value {
            NumType::I32 => 0x7F,
            NumType::I64 => 0x7E,
            NumType::F32 => 0x7D,
            NumType::F64 => 0x7C,
        }
    }
}

impl Scan for NumType {
    fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        let mut byte = [0; 1];
        cursor.read_exact(&mut byte)?;
        match byte {
            [0x7F] => Ok(Self::I32),
            [0x7E] => Ok(Self::I64),
            [0x7D] => Ok(Self::F32),
            [0x7C] => Ok(Self::F64),
            _ => Err("undefine numtype".into()),
        }
    }
}
