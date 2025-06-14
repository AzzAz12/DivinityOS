use limine::{
    memory_map::EntryType,
    request::{HhdmRequest, MemoryMapRequest},
};
use spin::Mutex;
pub use x86_64::{PhysAddr, VirtAddr};

use crate::println;

#[used]
#[unsafe(link_section = ".requests")]
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

static mut PHYS_OFFSET: u64 = 0;

pub fn hhdm_init() {
    unsafe {
        PHYS_OFFSET = HHDM_REQUEST.get_response().unwrap().offset();
    }
}

pub fn phys2virt(paddr: PhysAddr) -> VirtAddr {
    VirtAddr::new(paddr.as_u64() + unsafe { PHYS_OFFSET })
}

pub fn virt2phys(vaddr: VirtAddr) -> PhysAddr {
    PhysAddr::new(vaddr.as_u64() - unsafe { PHYS_OFFSET })
}

pub fn phys_offset() -> PhysAddr {
    unsafe { PhysAddr::new(PHYS_OFFSET) }
}
