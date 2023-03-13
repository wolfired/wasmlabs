use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;

use crate::scan::Scan;
use crate::types::FuncType;
use crate::types::Vector;
use crate::utils::uleb_decode;

use super::ID;

pub struct TypeSec {
    id: ID,
    fts: Vector<FuncType>,
}

impl Display for TypeSec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    {} {{\n", self.id)?;
        for (i, ft) in self.fts.iter().enumerate() {
            write!(f, "      {}: {},\n", i, ft)?
        }
        write!(f, "    }},\n")
    }
}

impl TypeSec {
    pub(crate) fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        let id = ID::scan(cursor)?;

        uleb_decode(cursor)?;
        Ok(Self {
            id,
            fts: { Vector::scan(cursor)? },
        })
    }
}
