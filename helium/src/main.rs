#![no_std]
#![no_main]
pub mod arch;
pub mod console;
pub mod heap;
pub mod pmem;

pub fn main() {
    println!("Hello Kernel!");
    pmem::hhdm_init();
    pmem::pmm_init();
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("Panic: {}", info);
    loop {}
}
