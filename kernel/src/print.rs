use core::fmt::{Write, self, Arguments};

use crate::uart;

struct Printer;

impl Write for Printer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.bytes().for_each(uart::write);
        Ok(())
    }
}

pub fn _print(args: Arguments<'_>) {
    Printer.write_fmt(args).expect("failed to print!");
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::print::_print(format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! println {
    () => {{
        $crate::print!("\n");
    }};
    ($($arg:tt)*) => {{
        $crate::print::_print(format_args_nl!($($arg)*));
    }};
}