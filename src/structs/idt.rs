//! https://wiki.osdev.org/Interrupt_Descriptor_Table
use crate::asm::lidt;

#[derive(Clone, Copy)]
#[repr(C)]
struct Entry {
    pointer_low: u16,
    segment_selector: u16,
    reserved: u8,
    flags: u8, // P(1), DPL(2), 0(1), Gate Type(4)
    pointer_high: u16,
}

impl Entry {
    pub const fn missing() -> Self {
        Self {
            pointer_low: 0,
            segment_selector: 0,
            reserved: 0,
            flags: 0,
            pointer_high: 0,
        }
    }
}

#[repr(C)]
pub struct InterruptDescriptorTable {
    entries: [Entry; 256],
}

impl InterruptDescriptorTable {
    pub const fn empty() -> Self {
        Self {
            entries: [Entry::missing(); 256],
        }
    }

    fn as_ptr(&self) -> u32 {
        self as *const _ as u32
    }

    pub fn load(&self) {
        lidt(InterruptDescriptorPointer {
            limit: (core::mem::size_of::<Self>() - 1) as u16,
            base: self.as_ptr(),
        });
    }
}

#[repr(C, packed)]
pub struct InterruptDescriptorPointer {
    pub limit: u16,
    pub base: u32,
}
