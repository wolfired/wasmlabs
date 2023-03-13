use std::error::Error;
use std::io::Cursor;
use std::io::Read;

pub fn uleb_decode(cursor: &mut Cursor<&[u8]>) -> Result<u32, Box<dyn Error>> {
    let mut byte = [0; 1];
    let mut shift = -7;
    let mut result = 0;
    loop {
        cursor.read_exact(&mut byte)?;
        result |= (byte[0] as u32 & 0b01111111) << {
            shift += 7;
            shift
        };
        if 0 == byte[0] as u32 & 0b10000000 {
            return Ok(result);
        }
    }
}
