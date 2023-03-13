use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;
use std::io::Read;

use crate::scan::Scan;

use super::ValType;

#[derive(Copy, Debug, Clone)]
pub enum Mut {
    Const = 0x00,
    Var = 0x01,
}

impl Display for Mut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Const => write!(f, "\"const\""),
            Self::Var => write!(f, "\"var\""),
        }
    }
}

impl From<Mut> for u8 {
    fn from(value: Mut) -> Self {
        match value {
            Mut::Const => 0x00,
            Mut::Var => 0x01,
        }
    }
}

impl Scan for Mut {
    fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        let mut byte = [0; 1];
        cursor.read_exact(&mut byte)?;
        match byte[0] {
            0x00 => Ok(Self::Const),
            0x01 => Ok(Self::Var),
            _ => Err("undefine mut".into()),
        }
    }
}

#[derive(Copy, Debug, Clone)]
pub struct GlobalType {
    t: ValType,
    m: Mut,
}

impl Display for GlobalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{m: {}, t: {}}}", self.m, self.t)
    }
}

impl Scan for GlobalType {
    fn scan(cursor: &mut Cursor<&[u8]>) -> Result<GlobalType, Box<dyn Error>> {
        Ok(GlobalType {
            t: ValType::scan(cursor)?,
            m: Mut::scan(cursor)?,
        })
    }
}
