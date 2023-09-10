use core::hint;
use crate::memlayout::UART0;

static mut HAS_INITED: bool = false;

pub fn is_available() -> bool {
    unsafe {HAS_INITED}
}

pub fn init() {
    // disable interrupt first
    ier::disable_interrupt();

    // set DLL & DLM mode
    lcr::set_dll_dlm_mode();

    // set baud rate
    dll_dlm::set_baud_rate(38400);

    // set RBR, THR, IER mode & 8 bits & no parity
    lcr::clear_dll_dlm_set_bits8_no_parity();

    // reset & enable fifo
    fcr::enable_and_reset_fifo();

    // enable interrupt
    ier::enable_rt_interrupt();

    unsafe {HAS_INITED = true;}
}

pub fn read() -> u8 {
    while !lsr::is_data_available() {
        hint::spin_loop();
    }
    rbr::read()
}

pub fn write(data: u8) {
    while !lsr::is_thr_empty() {
        hint::spin_loop();
    }
    thr::write(data);
}

fn read_reg(reg: usize) -> u8 {
    unsafe {((UART0 + reg) as *const u8).read_volatile()}
}

fn write_reg(reg: usize, data: u8) {
    unsafe {((UART0 + reg) as *mut u8).write_volatile(data)};
}

// interrupt enable register
mod ier {
    use super::write_reg;

    const REG: usize = 1;

    const RECEIVED_DATA_AVAILABLE: u8 = 1 << 0;
    const TRANSMITTER_HOLDING_REGISTER_EMPTY: u8 = 1 << 1;

    fn write(data: u8) {
        write_reg(REG, data);
    }

    pub fn disable_interrupt() {
        write(0);
    }

    pub fn enable_rt_interrupt() {
        write(RECEIVED_DATA_AVAILABLE | TRANSMITTER_HOLDING_REGISTER_EMPTY);
    }
}

// line control register
mod lcr {
    use super::write_reg;

    const REG: usize = 3;

    const DLL_DLM_MODE: u8 = 1 << 7;
    const BITS8: u8 = 3;

    fn write(data: u8) {
        write_reg(REG, data);
    }

    pub fn set_dll_dlm_mode() {
        write(DLL_DLM_MODE);
    }

    pub fn clear_dll_dlm_set_bits8_no_parity() {
        write(BITS8);
    }
}

// divisor latch register
mod dll_dlm {
    use super::write_reg;

    const DLL: usize = 0;
    const DLM: usize = 1;

    pub fn set_baud_rate(speed: u32) {
        let (dll, dlm) = match speed {
            115200 => (0x01, 0x00),
            57600  => (0x02, 0x00),
            38400  => (0x03, 0x00),
            19200  => (0x06, 0x00),
            9600   => (0x0c, 0x00),
            4800   => (0x18, 0x00),
            2400   => (0x30, 0x00),
            1200   => (0x60, 0x00),
            300    => (0x80, 0x01),
            50     => (0x00, 0x09),
            _      => panic!("unsupported speed: {speed}")
        };
        write_reg(DLL, dll);
        write_reg(DLM, dlm);
    }
}

// fifo control register
mod fcr {
    use super::write_reg;

    const REG: usize = 2;

    const ENABLE_FIFO: u8 = 1 << 0;
    const CLEAR_RECEIVE_FIFO: u8 = 1 << 1;
    const CLEAR_TRANSMIT_FIFO: u8 = 1 << 2;

    pub fn enable_and_reset_fifo() {
        write_reg(REG, ENABLE_FIFO | CLEAR_RECEIVE_FIFO | CLEAR_TRANSMIT_FIFO);
    }
}

// line status register
mod lsr {
    use super::read_reg;

    const REG: usize = 5;

    const DATA_AVAILABLE: u8 = 1 << 0;
    const THR_EMPTY: u8 = 3 << 5;

    fn read() -> u8 {
        read_reg(REG)
    }

    pub fn is_data_available() -> bool {
        read() & DATA_AVAILABLE != 0
    }

    pub fn is_thr_empty() -> bool {
        read() & THR_EMPTY != 0
    }
}

// receiver buffer register
mod rbr {
    use super::read_reg;

    const REG: usize = 0;

    pub fn read() -> u8 {
        read_reg(REG)
    }
}

// transmitter holding register
mod thr {
    use super::write_reg;
    
    const REG: usize = 0;

    pub fn write(data: u8) {
        write_reg(REG, data);
    }
}