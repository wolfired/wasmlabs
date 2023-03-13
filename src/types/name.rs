use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;
use std::io::Read;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::scan::Scan;
use crate::utils::uleb_decode;

pub struct Name(String);

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", &**self)
    }
}

impl Deref for Name {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Name {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Scan for Name {
    fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        let n = uleb_decode(cursor)?;
        let mut bytes = vec![0; n as usize];
        cursor.read_exact(&mut bytes)?;
        Ok(Name(String::from_utf8(bytes)?))
    }
}
