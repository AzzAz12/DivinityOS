#![no_std]
#![no_main]
pub mod arch;
pub mod console;

pub fn main() {
    println!("Hello Kernel!");
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("Panic: {}", info);
    loop {}
}
