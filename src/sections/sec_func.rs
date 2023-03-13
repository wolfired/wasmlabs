use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;

use crate::scan::Scan;
use crate::types::Index;
use crate::utils::uleb_decode;

use super::ID;

pub struct FuncSec {
    id: ID,
    xes: Vec<Index>,
}

impl Display for FuncSec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    {} {{\n", self.id)?;
        for (i, x) in self.xes.iter().enumerate() {
            write!(f, "      {}: {},\n", i, x)?
        }
        write!(f, "    }},\n")
    }
}

impl FuncSec {
    pub(crate) fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        let id = ID::scan(cursor)?;
        uleb_decode(cursor)?;
        Ok(Self {
            id,
            xes: {
                let count = uleb_decode(cursor)?;
                let mut xes = Vec::new();
                for _ in 0..count {
                    xes.push(Index::Typeidx(uleb_decode(cursor)?));
                }
                xes
            },
        })
    }
}
