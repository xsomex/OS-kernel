use common::x86_64::{
    set_general_handler,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

use common::lazy_static::lazy_static;

use crate::{println, Write, DISPLAY_TEXT};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        set_general_handler!(&mut idt, handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

fn handler(s: InterruptStackFrame, interrupt: u8, opt: Option<u64>) {
    println!("----------------------------------------");
    println!("INTERRUPT: {}", interrupt);
    println!("STACK_FRAME: {:?}", s);
    println!("ERROR CODE: {:?}", opt);
    println!("----------------------------------------");
    panic!()
}
