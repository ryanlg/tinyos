use core::cell::UnsafeCell;
use core::fmt;
use core::mem::MaybeUninit;
use core::ops::{Deref, DerefMut};

const TEXT_BUFFER_ADDRESS: i32 = 0xB8000;
const TEXT_BUFFER_WIDTH: usize = 80;
const TEXT_BUFFER_HEIGHT: usize = 25;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum TextColor {
    White = 15,
    // Other colors are omitted
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Character {
    ascii: u8,
    color: TextColor,
}

type CharacterGrid = [[Character; TEXT_BUFFER_WIDTH]; TEXT_BUFFER_HEIGHT];

#[repr(transparent)]
pub struct TextBuffer {
    grid: &'static mut CharacterGrid,
}

impl TextBuffer {
    pub fn new() -> Self {
        unsafe {
            TextBuffer {
                grid: &mut *(TEXT_BUFFER_ADDRESS as *mut CharacterGrid),
            }
        }
    }

    pub fn write(&mut self, row: u8, col: u8, ascii: u8, color: TextColor) {
        let c = Character { ascii, color };
        self.grid[row as usize][col as usize] = c;
    }

    pub fn scrollup(&mut self) {
        for r in 1..TEXT_BUFFER_HEIGHT {
            for c in 0..TEXT_BUFFER_WIDTH {
                self.grid[r - 1][c] = self.grid[r][c];
            }
        }
    }
}

pub struct Console {
    buffer: TextBuffer,

    // Next character goes here
    cur_row: u8,
    cur_col: u8,
}

impl Console {
    pub fn new() -> Self {
        Console {
            buffer: TextBuffer::new(),
            cur_row: 0,
            cur_col: 0,
        }
    }

    pub fn write(&mut self, ascii: u8, color: TextColor) {
        match ascii {
            b'\n' => self.write_newline(),
            _ => {
                self.buffer.write(self.cur_row, self.cur_col, ascii, color);
                self.advance();
            }
        }
    }

    pub fn write_newline(&mut self) {
        self.cur_row += 1;
        self.cur_col = 0;
        self.try_scrollup();
    }

    fn advance(&mut self) {
        self.cur_col += 1;
        if self.cur_col as usize >= TEXT_BUFFER_WIDTH {
            self.cur_col = 0;
            self.cur_row += 1;
        }
        self.try_scrollup();
    }

    fn try_scrollup(&mut self) {
        if self.cur_row as usize >= TEXT_BUFFER_HEIGHT {
            self.cur_row -= 1;
            self.buffer.scrollup();
        }
    }
}

// @attrib: https://github.com/ryanlg/Bedrock/blob/master/bootloader/src/arch/x86/core.rs
// I know, this is very ugly, but I didn't want to pull in `lazy_static` like
// phil-opp does, so this is a hack that I came up with a long time ago.
// Since then, `static mut` has been banished by Rust. I get why, but we are single-core
// only in this OS, so that lint is disabled.
pub static mut CONSOLE_GLOBAL: ConsoleGlobal = ConsoleGlobal {
    initialized: UnsafeCell::new(false),
    console: UnsafeCell::new(MaybeUninit::<Console>::uninit()),
};

/// Not thread safe at all
unsafe impl Sync for ConsoleGlobal {}

pub struct ConsoleGlobal {
    initialized: UnsafeCell<bool>,
    console: UnsafeCell<MaybeUninit<Console>>,
}

impl ConsoleGlobal {
    fn try_initialize(&self) {
        unsafe {
            if !*self.initialized.get() {
                (*self.console.get()).write(Console::new());
                *self.initialized.get() = true;
            }
        }
    }
}

impl core::fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            match byte {
                b'\n' => self.write_newline(),
                _ => self.write(byte, TextColor::White),
            }
        }
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    use crate::vga::CONSOLE_GLOBAL;
    use core::fmt::Write;
    unsafe {
        #[expect(static_mut_refs)]
        let _ = CONSOLE_GLOBAL.write_fmt(args);
    }
}

impl Deref for ConsoleGlobal {
    type Target = Console;
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
macro_rules! println {
    ()            => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}
