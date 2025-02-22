#![no_main]
#![no_std]
#![allow(non_snake_case)]
#![feature(sync_unsafe_cell, abi_x86_interrupt)]

use kernel::{
    DISPLAY_TEXT, Write, fill, interrupt, println, set_background_color,
    set_foreground_color, DisplayText
};

use common::bootloader_api::{BootloaderConfig, config::Mapping};

const CONFIG: BootloaderConfig = {
    let mut c = BootloaderConfig::new_default();
    c.mappings.page_table_recursive = Some(Mapping::Dynamic);
    c
};

fn start(info: &mut common::bootloader_api::BootInfo) -> ! {
    // if no display, panic (and probably triple fault later)
    match &mut info.framebuffer {
        common::bootloader_api::info::Optional::None => panic!(),
        common::bootloader_api::info::Optional::Some(f) => display_text::init(f),
    };

    fill!(0, 0, 0);
    println!("Kernel successfully loaded");
    interrupt::init_idt();
    set_background_color!(255, 0, 0);
    set_foreground_color!(0, 255, 0);
    println!("IDT loaded");
    memory::test();
    loop {}
}

common::bootloader_api::entry_point!(start, config = &CONFIG);
