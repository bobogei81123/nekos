use alloc::alloc::Layout;
use linked_list_allocator::LockedHeap;
use x86_64::{
    structures::paging::{
        mapper::{MapToError, Mapper},
        FrameAllocator, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};
use crate::memory::{
    frame::FRAME_ALLOCATOR,
    mapper::MAPPER,
};

pub const HEAP_START: usize = 0x0000_1000_0000_0000;
pub const HEAP_SIZE: usize = 1000 * 1024; // 100 KiB

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_heap() -> Result<(), MapToError<Size4KiB>> {
    let mut frame_allocator = FRAME_ALLOCATOR.lock();
    let mut mapper = MAPPER.lock();

    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, &mut *frame_allocator)?.flush() };
    }

    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
