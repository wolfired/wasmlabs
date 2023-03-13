use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;
use std::io::Read;

use crate::scan::Scan;
use crate::utils::uleb_decode;

use super::Index;
use super::Name;

pub struct Export {
    nm: Name,
    d: Index,
}

impl Display for Export {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{nm: {}, d: {}}}", self.nm, self.d)
    }
}

impl Scan for Export {
    fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        Ok(Self {
            nm: Name::scan(cursor)?,
            d: {
                let mut byte = [0; 1];
                cursor.read_exact(&mut byte)?;
                match byte[0] {
                    0x00 => Index::Funcidx(uleb_decode(cursor)?),
                    0x01 => Index::Tableidx(uleb_decode(cursor)?),
                    0x02 => Index::Memidx(uleb_decode(cursor)?),
                    0x03 => Index::Globalidx(uleb_decode(cursor)?),
                    _ => return Err("".into()),
                }
            },
        })
    }
}
