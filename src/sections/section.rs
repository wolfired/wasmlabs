use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;
use std::io::Seek;
use std::io::SeekFrom;

use crate::scan::Scan;

use super::CodeSec;
use super::CustomSec;
use super::DataCountSec;
use super::DataSec;
use super::ElemSec;
use super::ExportSec;
use super::FuncSec;
use super::GlobalSec;
use super::ImportSec;
use super::MemSec;
use super::StartSec;
use super::TableSec;
use super::TypeSec;
use super::ID;

pub enum Section {
    Custom(CustomSec),
    Type(TypeSec),
    Import(ImportSec),
    Function(FuncSec),
    Table(TableSec),
    Memory(MemSec),
    Global(GlobalSec),
    Export(ExportSec),
    Start(StartSec),
    Element(ElemSec),
    Code(CodeSec),
    Data(DataSec),
    DataCount(DataCountSec),
}

impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Custom(sec) => write!(f, "{}", sec),
            Self::Type(sec) => write!(f, "{}", sec),
            Self::Import(sec) => write!(f, "{}", sec),
            Self::Function(sec) => write!(f, "{}", sec),
            Self::Table(sec) => write!(f, "{}", sec),
            Self::Memory(sec) => write!(f, "{}", sec),
            Self::Global(sec) => write!(f, "{}", sec),
            Self::Export(sec) => write!(f, "{}", sec),
            Self::Start(sec) => write!(f, "{}", sec),
            Self::Element(sec) => write!(f, "{}", sec),
            Self::Code(sec) => write!(f, "{}", sec),
            Self::Data(sec) => write!(f, "{}", sec),
            Self::DataCount(sec) => write!(f, "{}", sec),
        }
    }
}

impl Section {
    pub(crate) fn scan(cursor: &mut Cursor<&[u8]>) -> Result<Self, Box<dyn Error>> {
        let id = ID::scan(cursor)?;
        cursor.seek(SeekFrom::Current(-1))?;
        match id {
            ID::Custom => Ok(Section::Custom(CustomSec::scan(cursor)?)),
            ID::Type => Ok(Section::Type(TypeSec::scan(cursor)?)),
            ID::Import => Ok(Section::Import(ImportSec::scan(cursor)?)),
            ID::Function => Ok(Section::Function(FuncSec::scan(cursor)?)),
            ID::Table => Ok(Section::Table(TableSec::scan(cursor)?)),
            ID::Memory => Ok(Section::Memory(MemSec::scan(cursor)?)),
            ID::Global => Ok(Section::Global(GlobalSec::scan(cursor)?)),
            ID::Export => Ok(Section::Export(ExportSec::scan(cursor)?)),
            ID::Start => Ok(Section::Start(StartSec::scan(cursor)?)),
            ID::Element => Ok(Section::Element(ElemSec::scan(cursor)?)),
            ID::Code => Ok(Section::Code(CodeSec::scan(cursor)?)),
            ID::Data => Ok(Section::Data(DataSec::scan(cursor)?)),
            ID::DataCount => Ok(Section::DataCount(DataCountSec::scan(cursor)?)),
        }
    }
}
