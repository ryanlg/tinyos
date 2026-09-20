/// Assembly wrapped in Rust.
use core::arch::asm;

/// Output one byte to port
pub fn outb(port: u16, byte: u8) {
    unsafe {
        asm!(
            "OUT dx, al",
            in("dx") port,
            in("al") byte,
        );
    }
}

/// Input one byte from port
pub fn inb(port: u16) -> u8 {
    let byte: u8;

    unsafe {
        asm!(
            "IN al, dx",
            in("dx") port,
            out("al") byte,
        );
    }

    byte
}
