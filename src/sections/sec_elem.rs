use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;
use std::io::Seek;
use std::io::SeekFrom;

use crate::scan::Scan;
use crate::utils::uleb_decode;

use super::ID;

pub enum Elem {
    
}

pub struct ElemSec {
    id: ID,
}

impl Display for ElemSec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    {} {{\n", self.id)?;
        write!(f, "    }},\n")
    }
}

impl ElemSec {
    pub(crate) fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        let id = ID::scan(cursor)?;
        let size = uleb_decode(cursor)?;
        cursor.seek(SeekFrom::Current(size as i64))?;
        Ok(Self { id })
    }
}
