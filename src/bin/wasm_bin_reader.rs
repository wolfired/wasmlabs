#![allow(unused_imports)]
#![allow(dead_code)]

use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;

use wasmlabs::module::Module;
use wasmlabs::scan::Scan;

fn main() -> Result<(), Box<dyn Error>> {
    let mut f = File::open("./res/lib.wasm")?;

    let mut raw = Vec::new();

    f.read_to_end(&mut raw)?;

    let mut cursor = Cursor::new(raw.as_slice());

    let m = Module::scan(&mut cursor)?;
    println!("{}", m);

    Ok(())
}
