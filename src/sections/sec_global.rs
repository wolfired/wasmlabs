use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;

use crate::scan::Scan;
use crate::types::Expr;
use crate::types::GlobalType;
use crate::types::Vector;
use crate::utils::uleb_decode;

use super::ID;

pub struct Global {
    gt: GlobalType,
    e: Expr,
}

impl Display for Global {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{gt: {}, e: {}}}", &self.gt, &self.e)
    }
}

impl Scan for Global {
    fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        Ok(Self {
            gt: GlobalType::scan(cursor)?,
            e: Expr::scan(cursor)?,
        })
    }
}

pub struct GlobalSec {
    id: ID,
    globs: Vector<Global>,
}

impl Display for GlobalSec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    {} {{\n", self.id)?;
        for (i, glob) in self.globs.iter().enumerate() {
            write!(f, "      {}: {},\n", i, &glob)?
        }
        write!(f, "    }},\n")
    }
}

impl GlobalSec {
    pub(crate) fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        let id = ID::scan(cursor)?;
        uleb_decode(cursor)?;
        Ok(Self {
            id,
            globs: Vector::scan(cursor)?,
        })
    }
}
