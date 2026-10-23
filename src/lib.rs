// @attrib: https://github.com/phil-opp/blog_os/blob/post-02/src/main.rs
#![no_std]
#![no_main]

mod asm;
mod idt;
mod serial;
mod structs;
mod vga;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    serial::init();
    dual_println!("\nRust start");

    println!("IDT init start");
    idt::init();
    println!("IDT init done");

    asm::fault_de();

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[macro_export]
macro_rules! dual_println {
    () => ($crate::dual_print!("\n"));
    ($($arg:tt)*) => ($crate::dual_print!("{}\n", ::core::format_args!($($arg)*)));
}

#[macro_export]
macro_rules! dual_print {
    ($($arg:tt)*) => {{
        $crate::vga::_print(format_args!($($arg)*));
        $crate::serial::_print(format_args!($($arg)*));
    }};
}
