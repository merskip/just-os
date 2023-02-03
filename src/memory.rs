use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{PhysAddr, structures::paging::{FrameAllocator, OffsetPageTable, PageTable, PhysFrame, Size4KiB}, VirtAddr};

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}

pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn new(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0
        }
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }   
}

impl BootInfoFrameAllocator {
    const FRAME_SIZE: usize = 4096;

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(
            |region| region.region_type == MemoryRegionType::Usable
        );
        let addresses_ranges = usable_regions.map(
            |region| region.range.start_addr()..region.range.end_addr()
        );
        let frames_addresses = addresses_ranges.flat_map(
            |range| range.step_by(Self::FRAME_SIZE)
        );
        frames_addresses.map(
            |frame_address| PhysFrame::containing_address(PhysAddr::new(frame_address))
        )
    }
}

