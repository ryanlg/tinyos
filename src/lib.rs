// @attrib: https://github.com/phil-opp/blog_os/blob/post-02/src/main.rs
#![no_std]
#![no_main]

mod asm;
mod serial;
mod vga;
mod structs;
mod idt;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    print!("Hello World{}", "!");
    serial_print!("Hello World{}", "!!!!");

    idt::init();

    print!("IDT init done");
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
