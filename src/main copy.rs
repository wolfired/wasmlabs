#![allow(unused_imports)]
#![allow(dead_code)]

use std::error::Error;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;

use wasm_bin_reader::module::Module;
use wasm_bin_reader::scan::Scan;

fn main() -> Result<(), Box<dyn Error>> {
    let mut f = File::open("../lib.wasm")?;

    let mut raw = Vec::new();

    f.read_to_end(&mut raw)?;

    let mut cursor = Cursor::new(raw.as_slice());

    let m = Module::scan(&mut cursor)?;
    println!("{}", m);

    Ok(())
}
