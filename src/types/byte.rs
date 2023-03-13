use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;
use std::io::Read;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::scan::Scan;

#[derive(Copy, Debug, Clone)]
pub struct Byte(u8);

impl Display for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &**self)
    }
}

impl Deref for Byte {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Byte {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Scan for Byte {
    fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        Ok(Byte({
            let mut byte = [0; 1];
            cursor.read_exact(&mut byte)?;
            byte[0]
        }))
    }
}
