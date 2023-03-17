use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;

use crate::scan::Scan;
use crate::utils::uleb_decode;

use super::Byte;
use super::Expr;
use super::Index;
use super::Vector;

pub struct Data {
    mode: u32,
    x: Option<Index>,
    e: Option<Expr>,
    b: Vector<Byte>,
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{mode: {}}}", &self.mode)
    }
}

impl Scan for Data {
    fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        let mode = uleb_decode(cursor)?;
        match mode {
            0x0 => Ok(Self {
                mode,
                x: None,
                e: Some(Expr::scan(cursor)?),
                b: Vector::scan(cursor)?,
            }),
            0x1 => Ok(Self {
                mode,
                x: None,
                e: None,
                b: Vector::scan(cursor)?,
            }),
            0x2 => Ok(Self {
                mode,
                x: Some(Index::Memidx(uleb_decode(cursor)?)),
                e: Some(Expr::scan(cursor)?),
                b: Vector::scan(cursor)?,
            }),
            _ => Err("undefine data".into()),
        }
    }
}
