use core::arch::wasm32;

use std::{
    alloc::{GlobalAlloc, Layout},
    cell::UnsafeCell,
};

const PAGE_SIZE: usize = 65536;

struct WasmAllocator {}

impl WasmAllocator {
    fn alloc(pages: usize) {
        let return_val = wasm32::memory_grow(0, pages);

        if return_val != usize::MAX {}
    }
}
