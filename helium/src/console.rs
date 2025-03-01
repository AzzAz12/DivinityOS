use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

pub struct WriterWrapper {
    is_early: bool,
    early_con: SerialPort,
}

use core::fmt::Result;
use core::fmt::Write;

impl Write for WriterWrapper {
    fn write_str(&mut self, s: &str) -> Result {
        if self.is_early {
            self.early_con.write_str(s)
        } else {
            /* TODO: Impl the usermode console */
            Ok(())
        }
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<WriterWrapper> = {
        let mut serial = unsafe { SerialPort::new(0x3F8) };
        serial.init();
        Mutex::new(WriterWrapper {
            is_early: true,
            early_con: serial,
        })
    };
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::console::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(
        concat!($fmt, "\n"), $($arg)*));
}
