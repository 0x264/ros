use core::{arch::{global_asm, asm}, hint::unreachable_unchecked};

use crate::{param::{STACK_SIZE, NCPU}, reg::{satp, medeleg, mideleg, sie, pmpaddr0, pmpcfg0, mstatus, mepc, mhartid, tp}, uart};

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
    // disable paging
    satp::write(0);

    // delegate all interrupts and exceptions to supervisor mode
    medeleg::set_all();
    mideleg::set_all();

    // enable external, timer, software interrupts
    sie::enable_ets_interrupts();

    // config PMP to let supervisor mode access all memory
    pmpaddr0::write(0x3fffffffffffff);
    pmpcfg0::write(0xf);

    // set previous mode to supervisor
    // we will then switch to s-mode with mret
    mstatus::set_mpp_s();

    // set s-mode entry point
    mepc::write(start as usize);

    // save hart id to tp register, so we can read it in s-mode
    mhartid::save2tp();

    init_timer();

    asm!("mret");

    unreachable_unchecked();
}

unsafe fn start() -> ! {
    if tp::read() == 0 {
        // main hart, do some config
        uart::init();
    } else {
        // todo
    }
    // todo
    loop {}
}

fn init_timer() {
    // todo
}