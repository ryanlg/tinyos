/// Assembly wrapped in Rust.
use core::arch::asm;

use crate::structs::idt::InterruptDescriptorPointer;

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

/// Load Iterrupt Descriptor Table Register
pub fn lidt(pointer: InterruptDescriptorPointer) {
    unsafe {
        asm!(
            "lidt [{}]",
            in(reg) &pointer,
            options(readonly, nostack),
        );
    }
}

/// Trigger #DE fault for testing
pub fn fault_de() {
    unsafe {
        asm!("mov dx, 0; div dx");
    }
}
