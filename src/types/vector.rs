use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::scan::Scan;
use crate::utils::uleb_decode;

pub struct Vector<T: Display + Scan>(Vec<T>);

impl<T: Display + Scan> Display for Vector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for e in self.iter() {
            write!(f, "{}, ", e)?;
        }
        write!(f, "]")
    }
}

impl<T: Display + Scan> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Display + Scan> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Display + Scan> Scan for Vector<T> {
    fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        let n = uleb_decode(cursor)?;
        Ok(Vector({
            let mut ts = Vec::new();
            for _ in 0..n {
                ts.push(T::scan(cursor)?);
            }
            ts
        }))
    }
}
