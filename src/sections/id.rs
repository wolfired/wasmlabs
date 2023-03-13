use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::io::Cursor;
use std::io::Read;

use crate::scan::Scan;

#[derive(Copy, Debug, Clone)]
pub enum ID {
    Custom = 0x00,
    Type = 0x01,
    Import = 0x02,
    Function = 0x03,
    Table = 0x04,
    Memory = 0x05,
    Global = 0x06,
    Export = 0x07,
    Start = 0x08,
    Element = 0x09,
    Code = 0x0a,
    Data = 0x0b,
    DataCount = 0x0c,
}

impl Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ID::Custom => write!(f, "Custom"),
            ID::Type => write!(f, "Type"),
            ID::Import => write!(f, "Import"),
            ID::Function => write!(f, "Function"),
            ID::Table => write!(f, "Table"),
            ID::Memory => write!(f, "Memory"),
            ID::Global => write!(f, "Global"),
            ID::Export => write!(f, "Export"),
            ID::Start => write!(f, "Start"),
            ID::Element => write!(f, "Element"),
            ID::Code => write!(f, "Code"),
            ID::Data => write!(f, "Data"),
            ID::DataCount => write!(f, "DataCount"),
        }
    }
}

impl From<ID> for u8 {
    fn from(value: ID) -> Self {
        match value {
            ID::Custom => 0x00,
            ID::Type => 0x01,
            ID::Import => 0x02,
            ID::Function => 0x03,
            ID::Table => 0x04,
            ID::Memory => 0x05,
            ID::Global => 0x06,
            ID::Export => 0x07,
            ID::Start => 0x08,
            ID::Element => 0x09,
            ID::Code => 0x0a,
            ID::Data => 0x0b,
            ID::DataCount => 0x0c,
        }
    }
}

impl Scan for ID {
    fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        let mut arr = [0; 1];
        cursor.read_exact(&mut arr)?;
        match arr[0] {
            0x00 => Ok(Self::Custom),
            0x01 => Ok(Self::Type),
            0x02 => Ok(Self::Import),
            0x03 => Ok(Self::Function),
            0x04 => Ok(Self::Table),
            0x05 => Ok(Self::Memory),
            0x06 => Ok(Self::Global),
            0x07 => Ok(Self::Export),
            0x08 => Ok(Self::Start),
            0x09 => Ok(Self::Element),
            0x0a => Ok(Self::Code),
            0x0b => Ok(Self::Data),
            0x0c => Ok(Self::DataCount),
            _ => Err("undefine section id".into()),
        }
    }
}
