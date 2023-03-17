use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;
use std::io::Seek;
use std::io::SeekFrom;

use crate::scan::Scan;
use crate::types::Byte;
use crate::types::Expr;
use crate::types::ValType;
use crate::types::Vector;
use crate::utils::uleb_decode;

use super::ID;

pub struct Locals {
    n: u32,
    t: ValType,
}

impl Display for Locals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{n: {}, t: {}}}", self.n, &self.t)
    }
}

impl Scan for Locals {
    fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        let n = uleb_decode(cursor)?;
        println!("locals n: {:x}", n);
        Ok(Self {
            n,
            t: ValType::scan(cursor)?,
        })
    }
}

pub struct Func {
    localses: Vector<Locals>,
    e: Expr,
}

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{localses: {}, e: {}}}", &self.localses, &self.e)
    }
}

impl Scan for Func {
    fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        Ok(Self {
            localses: Vector::scan(cursor)?,
            e: Expr::scan(cursor)?,
        })
    }
}

pub struct Code {
    size: u32,
    code: Func,
}

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{size: {}, code: {}}}", self.size, &self.code)
    }
}

impl Scan for Code {
    fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        let size = uleb_decode(cursor)?;
        println!("code size: {:#04x}", size);
        Ok(Self {
            size,
            code: Func::scan(cursor)?,
        })
    }
}

pub struct CodeSec {
    id: ID,
    codes: Vector<Code>,
}

impl Display for CodeSec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    {} {{\n", self.id)?;
        for (i, code) in self.codes.iter().enumerate() {
            write!(f, "      {}: {},\n", i, &code)?
        }
        write!(f, "    }},\n")
    }
}

impl CodeSec {
    pub(crate) fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        let id = ID::scan(cursor)?;
        let size = uleb_decode(cursor)?;
        println!("codesec size: {:#04x}", size);
        // cursor.seek(SeekFrom::Current(size as i64))?;
        Ok(Self {
            id,
            codes: Vector::scan(cursor)?,
        })
    }
}
