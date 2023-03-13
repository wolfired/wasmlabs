use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;

use crate::scan::Scan;
use crate::utils::uleb_decode;

use super::ID;

pub struct DataCountSec {
    id: ID,
    n: u32,
}

impl Display for DataCountSec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    {} {{n: {}}},\n", self.id, self.n)
    }
}

impl DataCountSec {
    pub(crate) fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        let id = ID::scan(cursor)?;
        uleb_decode(cursor)?;
        Ok(Self {
            id,
            n: uleb_decode(cursor)?,
        })
    }
}
