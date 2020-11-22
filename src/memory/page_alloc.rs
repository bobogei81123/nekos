use spin::Mutex;
use crate::common::once::UnsafeOnceCell;
use crate::memory::{
    frame::FRAME_ALLOCATOR,
    mapper::MAPPER,
};
use x86_64::{
    VirtAddr,
    structures::paging::{
        Page,
        page::PageRange,
        Size4KiB,
        PageTableFlags,
        mapper::Mapper as _,
    },
};

const START_VIRTADDR: VirtAddr = VirtAddr::new_truncate(0x0000_2000_0000_0000);

pub struct PageAllocator {
    current_page: Page<Size4KiB>,
}

impl PageAllocator {
    fn new() -> Self {
        PageAllocator {
            current_page: Page::containing_address(START_VIRTADDR),
        }
    }

    pub fn alloc_page(&mut self) -> Page {
        let page = self.current_page;
        self.current_page += 1;
        let mut frame_allocator = FRAME_ALLOCATOR.lock();
        let mut mapper = MAPPER.lock();
        let frame = frame_allocator.pop().expect("run out of frames");
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, &mut *frame_allocator).unwrap().flush(); }
        page
    }

    pub fn alloc_pages(&mut self, n: u64) -> PageRange {
        let start = self.current_page;
        self.current_page += n;
        let pages = Page::range(start, self.current_page);
        let mut frame_allocator = FRAME_ALLOCATOR.lock();
        let mut mapper = MAPPER.lock();
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        for page in pages {
            let frame = frame_allocator.pop().expect("run out of frames");
            println!("Mapping page = {:?}", page);
            unsafe { mapper.map_to(page, frame, flags, &mut *frame_allocator).unwrap().flush(); }
        }
        pages
    }
}

pub static PAGE_ALLOCATOR: UnsafeOnceCell<Mutex<PageAllocator>> = unsafe { 
    UnsafeOnceCell::new()
};

pub unsafe fn init() {
    PAGE_ALLOCATOR.init(Mutex::new(PageAllocator::new()));
}

