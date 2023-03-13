use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

use crate::scan::Scan;
use crate::sections::Section;

#[derive(Copy, Debug, Clone)]
pub struct Magic(u32);

impl Display for Magic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Magic: {:#010x}(\\0asm)", self.0)
    }
}

impl Scan for Magic {
    fn scan(value: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        let mut arr = [0; 4];
        value.read_exact(&mut arr)?;

        Ok(Magic(u32::from_be_bytes(arr)))
    }
}

#[derive(Copy, Debug, Clone)]
pub struct Version(u32);

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Version: {:#010x}", self.0)
    }
}

impl Scan for Version {
    fn scan(value: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        let mut arr = [0; 4];
        value.read_exact(&mut arr)?;

        Ok(Version(u32::from_le_bytes(arr)))
    }
}

pub struct Module {
    magic: Magic,
    version: Version,
    sections: Vec<Section>,
}

#[rustfmt::skip]
impl Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Module {{\n")?;
        write!(f, "  {},\n", self.magic)?;
        write!(f, "  {},\n", self.version)?;

        write!(f, "  Sections({}): [\n", self.sections.len())?;
        for section in self.sections.iter() {
            write!(f, "{}", section)?;
        }
        write!(f, "  ]\n")?;

        write!(f, "}}\n")
    }
}

impl Scan for Module {
    fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        let pos_end = cursor.seek(SeekFrom::End(0))?;
        cursor.rewind()?;
        Ok(Self {
            magic: Magic::scan(cursor)?,
            version: Version::scan(cursor)?,
            sections: {
                let mut sections = Vec::new();
                while pos_end > cursor.position() {
                    sections.push(Section::scan(cursor)?);
                }
                sections
            },
        })
    }
}
