use spin::{Mutex, Once};
use x86_64::{
    structures::paging::{
        OffsetPageTable,
        PageTable,
        Page,
        FrameAllocator,
        page_table::PageTableFlags,
        frame::PhysFrame,
        mapper::{
            Mapper as IMapper,
            MapperFlush,
            FlagUpdateError,
            MapToError,
            TranslateError,
            UnmapError,
            MapperFlushAll,
        },
        page::Size4KiB,
    },
    VirtAddr,
    PhysAddr,
};

const PHYS_OFFSET: u64 = 0x0000001000000000;

pub fn to_virt(phys: PhysAddr) -> VirtAddr {
    VirtAddr::new(phys.as_u64() + PHYS_OFFSET)
}

pub fn to_phys(virt: VirtAddr) -> PhysAddr {
    PhysAddr::new(virt.as_u64() - PHYS_OFFSET)
}

use crate::common::once::UnsafeOnceCell;

pub static MAPPER: UnsafeOnceCell<Mutex<OffsetPageTable>> = unsafe { UnsafeOnceCell::new() };

unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}

pub unsafe fn init(physical_memory_offset: VirtAddr) {
    let level_4_table = active_level_4_table(physical_memory_offset);
    MAPPER.init(Mutex::new(OffsetPageTable::new(level_4_table, physical_memory_offset)))
}

/*
impl IMapper<Size4KiB> for Mapper {
    unsafe fn map_to_with_table_flags<A: FrameAllocator<Size4KiB>>(
        &mut self,
        page: Page<Size4KiB>,
        frame: PhysFrame<Size4KiB>,
        flags: PageTableFlags,
        parent_table_flags: PageTableFlags,
        frame_allocator: &mut A,
    ) -> Result<MapperFlush<Size4KiB>, MapToError<Size4KiB>> {
        self.get().unwrap().lock().map_to_with_table_flags(
            page, frame, flags, parent_table_flags, frame_allocator
        )
    }

    fn unmap(&mut self, page: Page<Size4KiB>)
        -> Result<(PhysFrame<Size4KiB>, MapperFlush<Size4KiB>), UnmapError>
    {
        self.get().unwrap().lock().unmap(page)
    }

    unsafe fn update_flags(
        &mut self,
        page: Page<Size4KiB>,
        flags: PageTableFlags
    ) -> Result<MapperFlush<Size4KiB>, FlagUpdateError> {
        self.get().unwrap().lock().update_flags(page, flags)
    }

    unsafe fn set_flags_p4_entry(
        &mut self,
        page: Page<Size4KiB>,
        flags: PageTableFlags
    ) -> Result<MapperFlushAll, FlagUpdateError> {
        self.get().unwrap().lock().set_flags_p4_entry(page, flags)
    }

    unsafe fn set_flags_p3_entry(
        &mut self,
        page: Page<Size4KiB>,
        flags: PageTableFlags
    ) -> Result<MapperFlushAll, FlagUpdateError> {
        self.get().unwrap().lock().set_flags_p3_entry(page, flags)
    }

    unsafe fn set_flags_p2_entry(
        &mut self,
        page: Page<Size4KiB>,
        flags: PageTableFlags
    ) -> Result<MapperFlushAll, FlagUpdateError> {
        self.get().unwrap().lock().set_flags_p2_entry(page, flags)
    }

    fn translate_page(&self, page: Page<Size4KiB>) -> Result<PhysFrame<Size4KiB>, TranslateError> {
        self.get().unwrap().lock().translate_page(page)
    }
}
*/
