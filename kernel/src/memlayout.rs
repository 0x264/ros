pub const UART0: usize = 0x10000000;

extern "C" {
    pub fn _kernel_end();
}

pub const PHYTOP: usize = 0x80000000 + 128 * 1024 * 1024;