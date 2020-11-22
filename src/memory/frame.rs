use core::{
    mem,
    ptr::NonNull,
};
use crate::memory::{self, mapper::MAPPER};
use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{
    structures::paging::{
        mapper::{self, Mapper},
        FrameAllocator as IFrameAllocator,
        OffsetPageTable, Page, PageTable, PhysFrame, Size4KiB,
    },
    PhysAddr, VirtAddr,
};
use spin::{Mutex, Once};
use crate::common::once::UnsafeOnceCell;

struct Frame {
    addr: PhysAddr,
    next: Option<NonNull<Frame>>,
}

/// A FrameAllocator that returns usable frames from the bootloader's memory map.
pub struct FrameAllocator {
    frame_list: Option<NonNull<Frame>>,
}

impl FrameAllocator {
    /// Create a FrameAllocator from the passed memory map.
    ///
    /// This function is unsafe because the caller must guarantee that the passed
    /// memory map is valid. The main requirement is that all frames that are marked
    /// as `USABLE` in it are really unused.
    pub unsafe fn new(memory_map: &'static MemoryMap) -> Self {
        let frames = memory_map
            .iter()
            .filter(|r| r.region_type == MemoryRegionType::Usable)
            .map(|r| r.range.start_addr()..r.range.end_addr())
            .flat_map(|r| r.step_by(4096));

        let frames_n = frames.clone().count();

        const struct_frame_per_frame: usize = 4096 / mem::size_of::<Frame>();
        let need_frames =
            (frames_n + struct_frame_per_frame - 1)
            / struct_frame_per_frame;

        println!("Usable frames: {}\n", frames_n - need_frames);

        let bootstrap_addrs = frames
            .clone()
            .take(need_frames)
            .flat_map(|fr| (fr..fr+4096).step_by(mem::size_of::<Frame>()))
            .map(|x| memory::mapper::to_virt(PhysAddr::new(x)).as_mut_ptr::<Frame>());

        let usable_frames = frames.skip(need_frames).map(PhysAddr::new);
        let mut last_frame = None;

        for (frame, write_addr) in usable_frames.zip(bootstrap_addrs) {
            write_addr.write(Frame {
                addr: frame,
                next: last_frame,
            });
            last_frame = Some(NonNull::new(write_addr).unwrap());
        }

        FrameAllocator {
            frame_list: last_frame,
        }
    }

    pub fn pop(&mut self) -> Option<PhysFrame> {
        if let Some(mut frame) = self.frame_list {
            let frame = unsafe { frame.as_mut() };
            self.frame_list = frame.next;
            Some(PhysFrame::from_start_address(frame.addr).unwrap())
        } else {
            None
        }
    }
}

unsafe impl IFrameAllocator<Size4KiB> for FrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        self.pop()
    }
}

pub static FRAME_ALLOCATOR: UnsafeOnceCell<Mutex<FrameAllocator>> = unsafe { 
    UnsafeOnceCell::new()
};

pub unsafe fn init(memory_map: &'static MemoryMap) {
    FRAME_ALLOCATOR.init(Mutex::new(FrameAllocator::new(memory_map)))
}

/*
fn reserve_stack_memory(size_in_pages: u64) -> Page {
    use core::sync::atomic::{AtomicU64, Ordering};
    static STACK_ALLOC_NEXT: AtomicU64 = AtomicU64::new(0x_5555_5555_0000);
    let start_addr = VirtAddr::new(
        STACK_ALLOC_NEXT.fetch_add(size_in_pages * Page::<Size4KiB>::SIZE, Ordering::Relaxed),
    );
    Page::from_start_address(start_addr).expect("`STACK_ALLOC_NEXT` not page aligned")
}

pub fn alloc_stack(
    size_in_pages: u64,
) -> Result<StackBounds, mapper::MapToError<Size4KiB>> {
    use x86_64::structures::paging::PageTableFlags as Flags;

    let guard_page = reserve_stack_memory(size_in_pages + 1);
    let stack_start = guard_page + 1;
    let stack_end = stack_start + size_in_pages;
    let mut frame_allocator = FRAME_ALLOCATOR.lock();

    for page in Page::range(stack_start, stack_end) {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(mapper::MapToError::FrameAllocationFailed)?;

        let flags = Flags::PRESENT | Flags::WRITABLE;
        unsafe {
            MAPPER.lock().map_to(page, frame, flags, &mut *frame_allocator)?.flush();
        }
    }

    Ok(StackBounds::new(
        stack_start.start_address(),
        stack_end.start_address(),
    ))
}
*/
