use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;

use crate::scan::Scan;

use super::type_limit::Limit;

#[derive(Copy, Debug, Clone)]
pub struct MemType {
    lim: Limit,
}

impl Display for MemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{lim: {}}}", self.lim)
    }
}

impl Scan for MemType {
    fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            lim: Limit::scan(cursor)?,
        })
    }
}
