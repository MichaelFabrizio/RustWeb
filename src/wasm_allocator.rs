use core::arch::wasm32;
use core::ptr::null_mut;

use std::{
    alloc::{GlobalAlloc, Layout},
    cell::UnsafeCell,
};

const PAGE_SIZE: usize = 65536;

struct WasmAllocator {
    lead_ptr: *mut u8,
    tracking_ptr: *mut u8,
}

impl WasmAllocator {
    unsafe fn internal_alloc(pages: usize) -> *mut u8 {
        let ptr = wasm32::memory_grow(0, pages);

        if ptr != usize::MAX {
            let ptr = (ptr * PAGE_SIZE) as *mut u8;
            ptr
        } else {
            null_mut()
        }
    }
}

//unsafe impl GlobalAlloc for WasmAllocator {}
