use core::arch::naked_asm;
use core::cell::UnsafeCell;

use crate::dual_println;
use crate::structs::idt::{
    EntryGateType, InterruptDescriptorTable, InterruptStackFrame, InterruptStackFrameWithErrorCode,
};

struct IdtCell(UnsafeCell<InterruptDescriptorTable>);

// SAFETY: Not actually thread safe, but we are single-core only and access is one-and-done.
unsafe impl Sync for IdtCell {}

static IDT_GLOBAL: IdtCell = IdtCell(UnsafeCell::new(InterruptDescriptorTable::empty()));

macro_rules! handler {
    ($name: ident) => {{
        // https://wiki.osdev.org/Interrupt_Service_Routines#Two-Stage_Assembly_Wrapping
        // Nightly rustc now has `extern "x86-interrupt"`. I'm actually not sure if it supports
        // i386, but either way I'm not using it because this is more fun and dangerous.
        #[unsafe(naked)]
        extern "C" fn wrapped_handler(_: &InterruptStackFrame) {
            naked_asm!(
                "pushad",
                // This took me a while.
                // I tought i386 followed x86-64 and used registers to pass arguments,
                // but i386 actually pushes everything onto the stack. So we need to
                // find our frame's address after pushad and push it.
                // @todo: I think we need to align the stack according to
                // https://os.phil-opp.com/better-exception-messages/#16-byte-alignment.
                // This might break later.
                "lea edi, [0x20 + esp]",
                "push edi",
                "call {}",
                // Pop out pushed stack frame address
                "add esp, 4",
                "popad",
                "iretd",
                sym $name,
            );
        }

        wrapped_handler
    }}
}

pub fn init() {
    let ptr = IDT_GLOBAL.0.get();

    // SAFETY: caller provides exclusive access. Plus, we are single-core anyway.
    unsafe {
        let table = &mut *ptr;

        table.set_isr(0, EntryGateType::Trap, handler!(division_by_zero));
        table.set_isr(8, EntryGateType::Trap, handler!(double_fault));

        table.load();
    };
}

extern "C" fn division_by_zero(frame: &InterruptStackFrame) {
    dual_println!("FAULT: division by zero!");
    dual_println!("{}", frame);

    loop {}
}

extern "C" fn double_fault(frame: &InterruptStackFrameWithErrorCode) {
    dual_println!("FAULT: double fault!");
    dual_println!("{}", frame);

    loop {}
}
