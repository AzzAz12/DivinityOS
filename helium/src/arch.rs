use limine::BaseRevision;
use limine::request::EntryPointRequest;
use limine::request::{RequestsEndMarker, RequestsStartMarker};
use x86_64::instructions::hlt;
use x86_64::instructions::interrupts;

#[used]
#[unsafe(link_section = ".requests")]
static ENTRY_POINT_REQUEST: EntryPointRequest =
    EntryPointRequest::with_entry_point(EntryPointRequest::new(), _start);

#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[unsafe(no_mangle)]
extern "C" fn _start() -> ! {
    super::main();
    abort()
}

pub fn abort() -> ! {
    interrupts::disable();
    loop {
        hlt();
    }
}

#[allow(dead_code)]
pub fn idle() {
    loop {
        hlt();
    }
}
