use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
}

impl BumpAllocator {
    /// Create a new bump allocator with the given heap range.
    pub const fn new(heap_start: usize, heap_end: usize) -> Self {
        BumpAllocator {
            heap_start,
            heap_end,
            next: heap_start,
        }
    }

    /// Initialize the allocator with heap start and end.
    pub unsafe fn init(&mut self, heap_start: usize, heap_end: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_end;
        self.next = heap_start;
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let alloc_start = self.next;
        let alloc_end = alloc_start.checked_add(layout.size()).unwrap_or(usize::MAX);

        if alloc_end > self.heap_end {
            return null_mut(); // Out of memory.
        }

        self.next = alloc_end;
        alloc_start as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocators do not support deallocation.
    }
}
