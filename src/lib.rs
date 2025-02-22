#![no_std]
#![feature(sync_unsafe_cell, abi_x86_interrupt)]
#![allow(non_snake_case)]

pub mod interrupt;

pub use display_text::{println, print, set_background_color, set_foreground_color, fill, Write, DisplayText, DISPLAY_TEXT};

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{:?}", info);
    loop {}
}
