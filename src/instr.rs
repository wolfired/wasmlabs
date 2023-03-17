use crate::{scan::Scan, utils::uleb_decode};

pub enum Opcode {
    // Control Instructions
    Unreachable = 0x00,
    Nop = 0x01,
    Block = 0x02,
    Loop = 0x03,
    If = 0x04,
    Br = 0x0C,
    BrIf = 0x0D,
    BrTable = 0x0E,
    Return = 0x0F,
    Call = 0x10,
    CallIndirect = 0x11,
    // Parametric Instructions
    Drop = 0x1A,
    Select = 0x1B,
    Selects = 0x1C,
    // Reference Instructions
    RefNull = 0xD0,
    RefIsNull = 0xD1,
    RefFunc = 0xD2,
    // Variable Instructions
    LocalGet = 0x20,
    LocalSet = 0x21,
    LocalTee = 0x22,
    GlobalGet = 0x23,
    GlobalSet = 0x24,
    // Table Instructions
    TableGet = 0x25,
    TableSet = 0x26,
    // Memory Instructions
    I32Load = 0x28,
    I64Load = 0x29,
    F32Load = 0x2A,
    F64Load = 0x2B,
    I32Load8s = 0x2C,
    I32Load8u = 0x2D,
    I32Load16s = 0x2E,
    I32Load16u = 0x2F,
    I64Load8s = 0x30,
    I64Load8u = 0x31,
    I64Load16s = 0x32,
    I64Load16u = 0x33,
    I64Load32s = 0x34,
    I64Load32u = 0x35,
    I32Store = 0x36,
    I64Store = 0x37,
    F32Stroe = 0x38,
    F64Stroe = 0x39,
    I32Stroe8 = 0x3A,
    I32Stroe16 = 0x3B,
    I64Stroe8 = 0x3C,
    I64Stroe16 = 0x3D,
    I64Stroe32 = 0x3E,
    MemorySize = 0x3F,
    MemoryGrow = 0x40,
    // Numeric Instructions
    I32Const = 0x41,
    I64Const = 0x42,
    F32Const = 0x43,
    F64Const = 0x44,
    I32Eqz = 0x45,
    I32Eq = 0x46,
    I32Ne = 0x47,
    I32Lts = 0x48,
    I32Ltu = 0x49,
}

pub struct MemArg {
    a: u32,
    o: u32,
}

impl Scan for MemArg {
    fn scan(cursor: &mut std::io::Cursor<&[u8]>) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        Ok(Self {
            a: uleb_decode(cursor)?,
            o: uleb_decode(cursor)?,
        })
    }
}



pub enum ControlInstr {}

pub enum ReferenceInstr {}

pub enum ParametricInstr {}

pub enum VariableInstr {}

pub enum TableInstr {}

pub enum MemoryInstr {}

pub enum NumericInstr {}

pub enum VectorInstr {}
