use core::arch::global_asm;

use crate::param::{STACK_SIZE, NCPU};

#[repr(C, align(16))]
struct Stack([u8; STACK_SIZE * NCPU]);

#[no_mangle]
static mut STACK_BASE: Stack = Stack([0; STACK_SIZE * NCPU]);

global_asm!(
    include_str!("entry.s"),
    STACK_SIZE = const crate::param::STACK_SIZE
);

#[no_mangle]
unsafe extern "C" fn rust_start() -> ! {
    // todo
    loop {}
}