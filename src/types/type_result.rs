use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::scan::Scan;

use super::type_val::ValType;
use super::vector::Vector;

pub struct ResultType(Vector<ValType>);

impl Display for ResultType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &**self)
    }
}

impl Deref for ResultType {
    type Target = Vector<ValType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ResultType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Scan for ResultType {
    fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        Ok(Self(Vector::<ValType>::scan(cursor)?))
    }
}
