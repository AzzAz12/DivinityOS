use core::mem;
use core::slice;
use lazy_static::lazy_static;
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

#[used]
#[unsafe(link_section = ".requests")]
static MMAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

#[derive(Debug, PartialEq)]
pub enum MmapEntryType {
    Usable,
    Reserved,
    AcpiNVS,
    BadMem,
    Reclaimable,
    Kernel,
    Framebuffer,
    Unknown,
}

struct PageInfo {
    used: bool,
}

impl PageInfo {
    pub fn is_used(&self) -> bool 
}

pub struct FrameAllocator {
    frames: &'static mut [mem::MaybeUninit<PageInfo>],
}

impl FrameAllocator {
    pub fn print_nframes(&self) {
        println!("Usable frames: {}", self.frames.len());
    }
    pub fn init() -> Self {
        let resp = MMAP_REQUEST.get_response().unwrap();
        let entries = resp.entries();
        let mut usable_memories: u64 = 0;
        entries.iter().for_each(|e| {
            let (base, length, entry_type) = (e.base, e.length, e.entry_type);
            let entry_type: MmapEntryType = match entry_type {
                EntryType::USABLE => MmapEntryType::Usable,
                EntryType::RESERVED => MmapEntryType::Reserved,
                EntryType::ACPI_RECLAIMABLE => MmapEntryType::Reclaimable,
                EntryType::ACPI_NVS => MmapEntryType::AcpiNVS,
                EntryType::BAD_MEMORY => MmapEntryType::BadMem,
                EntryType::BOOTLOADER_RECLAIMABLE => MmapEntryType::Reclaimable,
                EntryType::KERNEL_AND_MODULES => MmapEntryType::Kernel,
                EntryType::FRAMEBUFFER => MmapEntryType::Framebuffer,
                _ => MmapEntryType::Unknown,
            };
            if entry_type == MmapEntryType::Usable {
                usable_memories += length;
            }
            println!(
                "Found Entry {:#?} at 0x{:016X} size {} KiB",
                entry_type,
                phys2virt(PhysAddr::new(base)).as_u64(),
                length / 1024
            );
        });
        println!("Usable Memories {} KiB", usable_memories / 1024);
        let bitmap_start = entries
            .iter()
            .filter(|e| e.length >= usable_memories)
            .nth(0)
            .unwrap()
            .base;
        let frames_ptr: *mut mem::MaybeUninit<PageInfo> =
            phys2virt(PhysAddr::new(bitmap_start)).as_mut_ptr();
        let frames = unsafe { slice::from_raw_parts_mut(frames_ptr, usable_memories as usize) };
        Self { frames }
    }
}

lazy_static! {
    pub static ref PFA: Mutex<FrameAllocator> = Mutex::new(FrameAllocator::init());
}

pub fn pmm_init() {
    PFA.lock().print_nframes();
}
