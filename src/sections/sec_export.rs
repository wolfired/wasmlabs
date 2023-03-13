use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;

use crate::scan::Scan;
use crate::types::Export;
use crate::types::Vector;
use crate::utils::uleb_decode;

use super::ID;

pub struct ExportSec {
    id: ID,
    ets: Vector<Export>,
}

impl Display for ExportSec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    {} {{\n", self.id)?;
        write!(f, "      ets({}): [\n", self.ets.len())?;
        for mt in self.ets.iter() {
            write!(f, "        {},\n", mt)?
        }
        write!(f, "    }},\n")
    }
}

impl ExportSec {
    pub(crate) fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        let id = ID::scan(cursor)?;
        uleb_decode(cursor)?;
        Ok(Self {
            id,
            ets: Vector::scan(cursor)?,
        })
    }
}
