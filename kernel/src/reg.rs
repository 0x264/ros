pub mod satp {
    use core::arch::asm;

    #[inline(always)]
    pub unsafe fn write(v: u64) {
        asm!("csrw satp, {}", in(reg) v);
    }
}

pub mod medeleg {
    use core::arch::asm;

    #[inline(always)]
    pub unsafe fn set_all() {
        asm!("csrw medeleg, {}", in(reg) 0xffff);
    }
}

pub mod mideleg {
    use core::arch::asm;

    #[inline(always)]
    pub unsafe fn set_all() {
        asm!("csrw mideleg, {}", in(reg) 0xffff);
    }
}

pub mod sie {
    use core::arch::asm;

    // enable external, timer, software interrupts
    #[inline(always)]
    pub unsafe fn enable_ets_interrupts() {
        let interrupt_bits = (1 << 9) | (1 << 5) | (1 << 1);
        asm!("csrs sie, {}", in(reg) interrupt_bits);
    }
}

pub mod pmpaddr0 {
    use core::arch::asm;

    #[inline(always)]
    pub unsafe fn write(v: u64) {
        asm!("csrw pmpaddr0, {}", in(reg) v);
    }
}

pub mod pmpcfg0 {
    use core::arch::asm;

    #[inline(always)]
    pub unsafe fn write(v: u64) {
        asm!("csrw pmpcfg0, {}", in(reg) v);
    }
}

pub mod mstatus {
    use core::arch::asm;

    #[inline(always)]
    pub unsafe fn read() -> u64 {
        let r;
        asm!("csrr {}, mstatus", out(reg) r);
        r
    }

    #[inline(always)]
    pub unsafe fn write(v: u64) {
        asm!("csrw mstatus, {}", in(reg) v);
    }

    #[inline(always)]
    pub unsafe fn set_mpp_s() {
        let mut v = read();
        v &= !(1 << 12);
        v |= 1 << 11;
        write(v);
    }
}

pub mod mepc {
    use core::arch::asm;

    #[inline(always)]
    pub unsafe fn write(v: usize) {
        asm!("csrw mepc, {}", in(reg) v);
    }
}

pub mod mhartid {
    use core::arch::asm;

    #[inline(always)]
    pub unsafe fn save2tp() {
        asm!("csrr tp, mhartid");
    }
}