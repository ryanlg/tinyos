/// UART serial tty for QEMU.
use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::ops::{Deref, DerefMut};
use core::fmt;

use crate::asm::{outb, inb};

const COM1_PORT: u16 = 0x3F8;  // Base address for COM1

#[derive(Debug)]
struct SerialPort(pub u16);

impl SerialPort {
    fn new(port: u16) -> Self {
        SerialPort(port)
    }

    fn outb(&self, offset: u16, byte: u8) {
        outb(self.0 + offset, byte);
    }

    fn inb(&self, offset: u16) -> u8 {
        inb(self.0 + offset)
    }

    fn latch_dlab(&self) {
        let cur_clr = self.inb(3);
        self.outb(3, cur_clr | 1 << 7);
    }

    fn clear_dlab(&self) {
        let cur_clr = self.inb(3);
        self.outb(3, cur_clr & !(1 << 7));
    }

    pub fn init(&self) {
        self.clear_dlab();

        // Disable UART interrupt
        self.outb(1, 0x0);

        // Set baud rate, we are using divisor of 1
        self.latch_dlab();
        self.outb(0, 0x1);
        self.outb(1, 0x0);
        self.clear_dlab();

        self.outb(3, 0b11);  // Set protocol: 8N1
        self.outb(2, 0xC7);  // FIFO
        self.outb(4, 0x0F);  // Enable modem
    }

    fn can_put(&self) -> bool {
        (self.inb(5) & (1 << 5)) != 0
    }

    pub fn put_ascii(&self, ascii: u8) {
        while !self.can_put() {}
        self.outb(0, ascii);
    }
}

#[derive(Debug)]
pub struct SerialConsole(SerialPort);

impl SerialConsole {
    pub fn new() -> Self {
        let console = SerialConsole(SerialPort::new(COM1_PORT));
        console.0.init();
        console
    }

    pub fn write(&self, ascii: u8) {
        self.0.put_ascii(ascii);
    }
}

impl core::fmt::Write for SerialConsole {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            self.write(byte);
        }
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use crate::serial::SERIAL_GLOBAL;
    unsafe {
        #[expect(static_mut_refs)]
        let _ = SERIAL_GLOBAL.write_fmt(args);
    }
}


// @attrib: https://github.com/ryanlg/Bedrock/blob/master/bootloader/src/arch/x86/core.rs
pub static mut SERIAL_GLOBAL: ConsoleGlobal = ConsoleGlobal {
    initialized: UnsafeCell::new(false),
    console: UnsafeCell::new(MaybeUninit::<SerialConsole>::uninit()),
};

/// Not thread safe at all
unsafe impl Sync for ConsoleGlobal {}

pub struct ConsoleGlobal {
    initialized: UnsafeCell<bool>,
    console: UnsafeCell<MaybeUninit<SerialConsole>>,
}

impl ConsoleGlobal {
    fn try_initialize(&self) {
        unsafe {
            if !*self.initialized.get() {
                (*self.console.get()).write(SerialConsole::new());
                *self.initialized.get() = true;
            }
        }
    }
}

impl Deref for ConsoleGlobal {
    type Target = SerialConsole;
    fn deref(&self) -> &Self::Target {
        self.try_initialize();
        unsafe { &*(*self.console.get()).as_ptr() }
    }
}

impl DerefMut for ConsoleGlobal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.try_initialize();
        unsafe { &mut *(*self.console.get()).as_mut_ptr() }
    }
}

#[macro_export]
macro_rules! serial_println {
    ()            => (serial_print!("\n"));
    ($($arg:tt)*) => (serial_print!("{}\n", format_args!($($arg)*)))
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ($crate::serial::_print(format_args!($($arg)*)));
}
