use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;

use crate::scan::Scan;
use crate::types::TableType;
use crate::types::Vector;
use crate::utils::uleb_decode;

use super::ID;

pub struct TableSec {
    id: ID,
    tabs: Vector<TableType>,
}

impl Display for TableSec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    {} {{\n", self.id)?;
        for (i, tab) in self.tabs.iter().enumerate() {
            write!(f, "      {}: {},\n", i, &tab)?
        }
        write!(f, "    }},\n")
    }
}

impl TableSec {
    pub(crate) fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        let id = ID::scan(cursor)?;
        uleb_decode(cursor)?;
        Ok(Self {
            id,
            tabs: Vector::scan(cursor)?,
        })
    }
}
