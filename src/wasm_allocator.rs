use core::ptr::null_mut;
use std::alloc::{GlobalAlloc, Layout};

use super::{console_log, log};

const PAGE_SIZE: usize = 65536;

// Consider changing visibilities to pub(crate)
pub(crate) struct WasmAllocator {
    pub lead_ptr: *mut u8,
    pub tracking_ptr: *mut u8,
    pub allocation_size: usize,
}

impl WasmAllocator {
    pub(crate) fn memory_size() -> usize {
        core::arch::wasm32::memory_size(0)
    }

    pub(crate) unsafe fn internal_alloc(&mut self, pages: usize) -> *mut u8 {
        let ptr = core::arch::wasm32::memory_grow(0, pages);

        if ptr != usize::MAX {
            self.allocation_size += pages * PAGE_SIZE;
            let ptr = (ptr * PAGE_SIZE) as *mut u8;
            ptr
        } else {
            null_mut()
        }
    }

    pub(crate) fn debug_allocation_size(&self) {
        console_log!("allocation_size {} bytes", self.allocation_size);
    }
}

impl Default for WasmAllocator {
    fn default() -> Self {
        // The starting pointer value where we can begin constructing objects
        // This ignores the initial allocated pages created before our process starts...
        let allocated_start_pointer = (WasmAllocator::memory_size() * PAGE_SIZE) as *mut u8;

        WasmAllocator {
            lead_ptr: allocated_start_pointer,
            tracking_ptr: allocated_start_pointer,
            allocation_size: 0,
        }
    }
}

unsafe impl GlobalAlloc for WasmAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        console_log!("Layout size {}", layout.size());
        null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {}
}
