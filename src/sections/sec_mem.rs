use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;

use crate::scan::Scan;
use crate::types::MemType;
use crate::types::Vector;
use crate::utils::uleb_decode;

use super::ID;

pub struct MemSec {
    id: ID,
    mems: Vector<MemType>,
}

impl Display for MemSec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    {} {{\n", self.id)?;
        for (i, mem) in self.mems.iter().enumerate() {
            write!(f, "      {}: {},\n", i, &mem)?
        }
        write!(f, "    }},\n")
    }
}

impl MemSec {
    pub(crate) fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        let id = ID::scan(cursor)?;
        uleb_decode(cursor)?;
        Ok(Self {
            id,
            mems: Vector::scan(cursor)?,
        })
    }
}
