use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;

use crate::scan::Scan;
use crate::types::Data;
use crate::types::Vector;
use crate::utils::uleb_decode;

use super::ID;

pub struct DataSec {
    id: ID,
    ds: Vector<Data>,
}

impl Display for DataSec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    {} {{\n", self.id)?;
        for d in self.ds.iter() {
            write!(f, "        {},\n", d)?
        }
        write!(f, "    }},\n")
    }
}

impl DataSec {
    pub(crate) fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        let id = ID::scan(cursor)?;
        uleb_decode(cursor)?;
        Ok(Self {
            id,
            ds: Vector::scan(cursor)?,
        })
    }
}
