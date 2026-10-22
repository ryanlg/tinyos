//! https://wiki.osdev.org/Interrupt_Descriptor_Table
use core::fmt;

use crate::asm::lidt;

pub enum EntryGateType {
    Interrupt,
    Trap,
    // @todo: Abort?
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct EntryFlags(u8);

impl Default for EntryFlags {
    /// Default IDT entry: enabled, ring 0, trap gate
    fn default() -> Self {
        Self(0b1_00_0_1111)
    }
}

impl EntryFlags {
    pub fn new() -> Self {
        Self::default()
    }

    pub const fn missing() -> Self {
        Self(0)
    }

    pub fn set_gate_type(&mut self, gate_type: EntryGateType) -> &mut Self {
        match gate_type {
            EntryGateType::Interrupt => self.0 &= !0b1,
            EntryGateType::Trap => self.0 |= 0b1,
        }
        self
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
struct Entry {
    pointer_low: u16,
    segment_selector: u16,
    reserved: u8,
    flags: EntryFlags,
    pointer_high: u16,
}

impl Entry {
    pub const fn missing() -> Self {
        Self {
            pointer_low: 0,
            segment_selector: 0,
            reserved: 0,
            flags: EntryFlags::missing(),
            pointer_high: 0,
        }
    }

    pub fn new(isr: IsrFunc, gate_type: EntryGateType) -> Self {
        let ptr = isr as u32;
        Self {
            pointer_low: ptr as u16,
            segment_selector: 0x08, // code segment
            reserved: 0,
            flags: *EntryFlags::new().set_gate_type(gate_type),
            pointer_high: (ptr >> 16) as u16,
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

    pub fn set_isr(&mut self, number: u8, gate_type: EntryGateType, isr: IsrFunc) {
        self.entries[number as usize] = Entry::new(isr, gate_type);
    }
}

#[repr(C, packed)]
pub struct InterruptDescriptorPointer {
    pub limit: u16,
    pub base: u32,
}

#[repr(C)]
pub struct InterruptStackFrame {
    ip: u32,
    cs: u16,
    flags: u32,
}

impl fmt::Display for InterruptStackFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r"INTERRUPT FRAME:
    IP: {0:#X}
    CS: {1:#X}
    FLAGS: {2:#X}
",
            self.ip, self.cs, self.flags
        )
    }
}

type IsrFunc = extern "C" fn(&InterruptStackFrame);
