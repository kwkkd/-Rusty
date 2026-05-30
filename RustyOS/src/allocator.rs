use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;
use spin::Mutex;

pub struct Allocator {
    inner: Mutex<linked_list_allocator::Heap>,
}

impl Allocator {
    pub const fn new() -> Self {
        Allocator {
            inner: Mutex::new(linked_list_allocator::Heap::empty()),
        }
    }

    pub unsafe fn init(&self, heap_start: usize, heap_size: usize) {
        self.inner.lock().init(heap_start, heap_size);
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match self.inner.lock().allocate_first_fit(layout) {
            Ok(non_null) => non_null.as_mut_ptr(),
            Err(_) => core::ptr::null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.inner
            .lock()
            .deallocate(NonNull::new_unchecked(ptr), layout)
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
