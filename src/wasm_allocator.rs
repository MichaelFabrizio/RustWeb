//use core::arch::wasm32;
use core::ptr::null_mut;

use std::alloc::{GlobalAlloc, Layout};

const PAGE_SIZE: usize = 65536;

pub struct WasmAllocator {
    lead_ptr: *mut u8,
    tracking_ptr: *mut u8,
}

impl WasmAllocator {
    pub fn memory_size() -> usize {
        core::arch::wasm32::memory_size(0)
    }

    pub unsafe fn internal_alloc(pages: usize) -> *mut u8 {
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
