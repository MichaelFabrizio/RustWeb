use std::mem::align_of;

use super::{console_log, log};
use crate::keyvector::KeyVector;
use crate::wasm_allocator::WasmAllocator;

pub(super) struct WebCore {
    wasm_allocator: WasmAllocator,
}

impl WebCore {
    pub(super) fn new() -> Self {
        let mut wasm_allocator = WasmAllocator {
            ..Default::default()
        };

        unsafe {
            wasm_allocator.internal_alloc(2);
        }
        wasm_allocator.debug_allocation_size();

        WebCore { wasm_allocator }
    }

    pub(super) fn init(&mut self) {
        let returned_ptr = unsafe { self.wasm_allocator.internal_alloc(1) };
        console_log!("Returned ptr: {:?}", returned_ptr);
    }

    pub(super) fn addkeyvec<T>(&mut self) {
        let test_ptr = self.wasm_allocator.tracking_ptr as usize;

        //        let test_key_vec = KeyVector::<T, u8, 50> {};
        //        let keyvec_alignment = align_of();
    }

    pub(super) fn start_frame() {}

    pub(super) fn end_frame() {}
}
