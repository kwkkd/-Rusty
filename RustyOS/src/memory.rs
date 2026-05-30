use x86_64::{
    structures::paging::{FrameAllocator, Mapper, OffsetPageTable, PageTable, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};
use bootloader::bootinfo::{MemoryRegion, MemoryRegionType};
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref MAPPER: Mutex<Option<OffsetPageTable<'static>>> = Mutex::new(None);
    pub static ref ALLOCATOR: Mutex<BumpAllocator> = Mutex::new(BumpAllocator::new());
}

pub fn init(
    physical_memory_offset: VirtAddr,
    memory_regions: &'static [MemoryRegion],
) -> OffsetPageTable<'static> {
    let level_4_table = unsafe { &mut *(physical_memory_offset.as_mut_ptr::<PageTable>()) };
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
}

impl BumpAllocator {
    pub fn new() -> Self {
        Self {
            heap_start: 0,
            heap_end: 0,
            next: 0,
        }
    }

    pub fn init(&mut self, memory_regions: &[MemoryRegion]) {
        let usable_regions = memory_regions
            .iter()
            .filter(|r| r.region_type == MemoryRegionType::Usable);

        let max_addr = usable_regions
            .clone()
            .map(|r| r.range.end_frame_number * 0x1000)
            .max()
            .unwrap_or(0);

        self.heap_start = 0x_4444_4444_0000;
        self.heap_end = self.heap_start + 10 * 1024 * 1024; // 10 MB heap
        self.next = self.heap_start;
    }

    pub unsafe fn alloc(&mut self, layout: core::alloc::Layout) -> *mut u8 {
        let align_mask = layout.align() - 1;
        let aligned_next = (self.next + align_mask) & !align_mask;

        if aligned_next + layout.size() > self.heap_end {
            return core::ptr::null_mut();
        }

        let ret = aligned_next as *mut u8;
        self.next = aligned_next + layout.size();
        ret
    }

    pub unsafe fn dealloc(&mut self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        // Bump allocator doesn't support deallocation
    }
}

unsafe impl core::alloc::GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let mut allocator = ALLOCATOR.lock();
        allocator.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let mut allocator = ALLOCATOR.lock();
        allocator.dealloc(ptr, layout);
    }
}

pub struct BootInfoFrameAllocator {
    memory_map: &'static [MemoryRegion],
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_map: &'static [MemoryRegion]) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        let addr_ranges = usable_regions.map(|r| r.range.start_frame_number..r.range.end_frame_number);
        let frame_numbers = addr_ranges.flat_map(|r| r);
        frame_numbers.map(|frame_number| PhysFrame::containing_address(PhysAddr::new(frame_number * 0x1000)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
