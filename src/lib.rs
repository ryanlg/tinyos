// @attrib: https://github.com/phil-opp/blog_os/blob/post-02/src/main.rs
#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    print!("Hello World{}", "!");
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
