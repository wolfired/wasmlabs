use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;

use crate::scan::Scan;
use crate::types::Index;
use crate::utils::uleb_decode;

use super::ID;

pub struct StartSec {
    id: ID,
    x: Index,
}

impl Display for StartSec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    {} {{{}}}\n", self.id, &self.x)
    }
}

impl StartSec {
    pub(crate) fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        let id = ID::scan(cursor)?;
        uleb_decode(cursor)?;
        Ok(Self {
            id,
            x: Index::Funcidx(uleb_decode(cursor)?),
        })
    }
}
