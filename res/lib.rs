#![no_std]
#![allow(unused)]

use core::{arch::wasm32, panic::PanicInfo};

#[panic_handler]
fn phandler(_: &PanicInfo<'_>) -> ! {
    wasm32::unreachable();
}

extern "C" {
    fn vline(x: i32, y: i32, len: u32);
}

#[no_mangle]
unsafe fn max(x: i8, y: i8) -> i8 {
    let a = x + y;
    if a > y {
        x
    } else {
        y
    }
}

#[no_mangle]
unsafe fn min(x: i8, y: i8) -> i8 {
    if x > y {
        x
    } else {
        y
    }
}

#[no_mangle]
unsafe fn minf(x: f32, y: f32, z: f64) -> f32 {
    vline(0, 0, 0);
    if x > y {
        x
    } else {
        y
    }
}
