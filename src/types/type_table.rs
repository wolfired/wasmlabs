use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;

use crate::scan::Scan;

use super::type_limit::Limit;
use super::type_ref::RefType;

#[derive(Copy, Debug, Clone)]
pub struct TableType {
    et: RefType,
    lim: Limit,
}

impl Display for TableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{lim: {}, et: {}}}", &self.lim, &self.et)
    }
}

impl Scan for TableType {
    fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            et: RefType::scan(cursor)?,
            lim: Limit::scan(cursor)?,
        })
    }
}
