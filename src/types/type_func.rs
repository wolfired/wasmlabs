use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;
use std::io::Read;

use crate::scan::Scan;

use super::type_result::ResultType;

pub struct FuncType {
    rt1: ResultType,
    rt2: ResultType,
}

impl Display for FuncType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{rt1: {}, rt2: {}}}", self.rt1, self.rt2)
    }
}

impl Scan for FuncType {
    fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        let mut byte = [0; 1];
        cursor.read_exact(&mut byte)?;
        if [0x60] != byte {
            Err("undefine functype".into())
        } else {
            Ok(Self {
                rt1: ResultType::scan(cursor)?,
                rt2: ResultType::scan(cursor)?,
            })
        }
    }
}
