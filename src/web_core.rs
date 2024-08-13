use super::{console_log, log};
use crate::wasm_allocator::WasmAllocator;

pub(super) struct WebCore {
    wasm_allocator: WasmAllocator,
}

impl WebCore {
    pub(super) fn new() -> Self {
        let wasm_allocator = WasmAllocator {
            lead_ptr: core::ptr::null_mut(),
            tracking_ptr: core::ptr::null_mut(),
        };

        WebCore { wasm_allocator }
    }

    pub(super) fn init() {
        let returned_ptr = unsafe { WasmAllocator::internal_alloc(1) };
        console_log!("Returned ptr: {:?}", returned_ptr);
    }

    pub(super) fn start_frame() {}

    pub(super) fn end_frame() {}
}
