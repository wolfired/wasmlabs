use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;
use std::io::Read;

use crate::scan::Scan;
use crate::types::Name;
use crate::utils::uleb_decode;

use super::ID;

pub struct CustomSec {
    id: ID,
    name: Name,
    bytes: Vec<u8>,
}

impl Display for CustomSec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    {} {{", self.id)?;

        write!(f, "name: {}, bytes_len: {}", self.name, self.bytes.len())?;

        write!(f, "}},\n")
    }
}

impl CustomSec {
    pub(crate) fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        let id = ID::scan(cursor)?;

        let size = uleb_decode(cursor)?;
        let pos_end = cursor.position() + size as u64;

        let name = Name::scan(cursor)?;

        let bytes_size = pos_end - cursor.position();

        let mut bytes = vec![0; bytes_size as usize];
        cursor.read_exact(&mut bytes)?;

        Ok(Self { id, name, bytes })
    }
}
