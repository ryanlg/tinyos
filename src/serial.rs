/// UART serial tty for QEMU.
use core::cell::UnsafeCell;
use core::fmt;

use crate::asm::{inb, outb};

const COM1_PORT: u16 = 0x3F8; // Base address for COM1

#[derive(Debug)]
struct SerialPort(pub u16);

impl SerialPort {
    const fn new(port: u16) -> Self {
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

        self.outb(3, 0b11); // Set protocol: 8N1
        self.outb(2, 0xC7); // FIFO
        self.outb(4, 0x0F); // Enable modem
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
    pub const fn new() -> Self {
        SerialConsole(SerialPort::new(COM1_PORT))
    }

    pub fn init(&self) {
        self.0.init();
    }

    pub fn write(&self, ascii: u8) {
        self.0.put_ascii(ascii);
    }
}

impl fmt::Write for SerialConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write(byte);
        }
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    let ptr = SERIAL_GLOBAL.0.get();

    unsafe {
        let console = &mut *ptr;
        let _ = fmt::Write::write_fmt(console, args);
    }
}

struct SerialConsoleCell(UnsafeCell<SerialConsole>);
// SAFETY: Not actually thread safe, but we are single-core only.
unsafe impl Sync for SerialConsoleCell {}

static SERIAL_GLOBAL: SerialConsoleCell = SerialConsoleCell(UnsafeCell::new(SerialConsole::new()));

pub fn init() {
    let ptr = SERIAL_GLOBAL.0.get();

    unsafe {
        let console = &mut *ptr;
        console.init();
    }
}

#[macro_export]
macro_rules! serial_println {
    ()            => (use crate::serial_print; serial_print!("\n"));
    ($($arg:tt)*) => (use crate::serial_print; serial_print!("{}\n", format_args!($($arg)*)))
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ($crate::serial::_print(format_args!($($arg)*)));
}
