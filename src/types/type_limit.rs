use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;
use std::io::Read;

use crate::scan::Scan;
use crate::utils::uleb_decode;

#[derive(Copy, Debug, Clone)]
pub enum Limit {
    Floor { min: u32 },
    FloorRange { min: u32, max: u32 },
}

impl Display for Limit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Floor { min } => write!(f, "{{min: {min:#x}}}"),
            Self::FloorRange { min, max } => {
                write!(f, "{{min: {min:#x}, max: {max:#x}}}")
            }
        }
    }
}

impl Scan for Limit {
    fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        let mut byte = [0; 1];
        cursor.read_exact(&mut byte)?;
        match byte[0] {
            0x00 => Ok(Self::Floor {
                min: uleb_decode(cursor)?,
            }),
            0x01 => Ok(Self::FloorRange {
                min: uleb_decode(cursor)?,
                max: uleb_decode(cursor)?,
            }),
            _ => Err("undefine limit".into()),
        }
    }
}
