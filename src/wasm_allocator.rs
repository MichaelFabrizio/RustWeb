use core::ptr::null_mut;
use std::alloc::{GlobalAlloc, Layout};

use super::{console_log, log};

const PAGE_SIZE: usize = 65536;

// Consider changing visibilities to pub(crate)
pub(crate) struct WasmAllocator {
    pub lead_ptr: *mut u8,
    pub tracking_ptr: *mut u8,
}

impl WasmAllocator {
    pub(crate) fn memory_size() -> usize {
        core::arch::wasm32::memory_size(0)
    }

    pub(crate) unsafe fn internal_alloc(pages: usize) -> *mut u8 {
        let ptr = core::arch::wasm32::memory_grow(0, pages);

        if ptr != usize::MAX {
            let ptr = (ptr * PAGE_SIZE) as *mut u8;
            ptr
        } else {
            null_mut()
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
