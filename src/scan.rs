use std::error::Error;
use std::io::Cursor;

pub trait Scan {
    fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
}