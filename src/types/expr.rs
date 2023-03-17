use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;
use std::io::Read;

use crate::scan::Scan;

pub struct Expr {}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "e")
    }
}

impl Scan for Expr {
    fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        println!("");
        let mut byte = [0; 1];
        loop {
            cursor.read_exact(&mut byte)?;
            print!("{:#04x} ", byte[0]);
            if [0x0B] == byte {
                break;
            }
        }
        println!("");
        Ok(Self {})
    }
}
