use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;
use std::io::Read;

use crate::scan::Scan;
use crate::types::GlobalType;
use crate::types::Index;
use crate::types::MemType;
use crate::types::Name;
use crate::types::TableType;
use crate::types::Vector;
use crate::utils::uleb_decode;

use super::ID;

#[derive(Copy, Debug, Clone)]
pub enum ImportDesc {
    Index(Index),
    TableType(TableType),
    MemType(MemType),
    GlobalType(GlobalType),
}

impl Display for ImportDesc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Index(index) => write!(f, "{}", index),
            Self::TableType(tabletype) => write!(f, "{}", tabletype),
            Self::MemType(memtype) => write!(f, "{}", memtype),
            Self::GlobalType(globaltype) => write!(f, "{}", globaltype),
        }
    }
}

impl Scan for ImportDesc {
    fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        let mut byte = [0; 1];
        cursor.read_exact(&mut byte)?;
        match byte {
            [0x00] => Ok(Self::Index(Index::Typeidx(uleb_decode(cursor)?))),
            [0x01] => Ok(Self::TableType(TableType::scan(cursor)?)),
            [0x02] => Ok(Self::MemType(MemType::scan(cursor)?)),
            [0x03] => Ok(Self::GlobalType(GlobalType::scan(cursor)?)),
            _ => Err("undefine importdesc".into()),
        }
    }
}

pub struct Import {
    module: Name,
    nm: Name,
    d: ImportDesc,
}

impl Display for Import {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{mod: {}, nm: {}, d: {}}}",
            self.module, self.nm, self.d
        )
    }
}

impl Scan for Import {
    fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        Ok(Self {
            module: Name::scan(cursor)?,
            nm: Name::scan(cursor)?,
            d: ImportDesc::scan(cursor)?,
        })
    }
}

pub struct ImportSec {
    id: ID,
    ims: Vector<Import>,
}

impl Display for ImportSec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    {} {{\n", self.id)?;
        for (i, im) in self.ims.iter().enumerate() {
            write!(f, "      {}: {},\n", i, im)?
        }
        write!(f, "    }},\n")
    }
}

impl ImportSec {
    pub(crate) fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        let id = ID::scan(cursor)?;
        uleb_decode(cursor)?;
        Ok(Self {
            id,
            ims: Vector::scan(cursor)?,
        })
    }
}
