use core::cell::UnsafeCell;

use crate::structs::idt::{EntryGateType, InterruptDescriptorTable};
use crate::{print, println};

struct IdtCell(UnsafeCell<InterruptDescriptorTable>);

// SAFETY: Not actually thread safe, but we are single-core only and access is one-and-done.
unsafe impl Sync for IdtCell {}

static IDT_GLOBAL: IdtCell = IdtCell(UnsafeCell::new(InterruptDescriptorTable::empty()));

pub fn init() {
    let ptr = IDT_GLOBAL.0.get();

    // SAFETY: caller provides exclusive access. Plus, we are single-core anyway.
    unsafe {
        let table = &mut *ptr;

        table.set_isr(0, EntryGateType::Trap, division_by_zero);

        table.load();
    };
}

extern "C" fn division_by_zero() -> ! {
    println!("EXCEPTION: division by zero!");

    loop {}
}
