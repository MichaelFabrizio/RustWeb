use std::mem::{align_of, size_of};
use std::ptr::addr_of_mut;

use super::{console_log, log};
use crate::wasm_allocator::WasmAllocator;

pub(crate) trait IndexVariable: Copy {
    const MAX_VALUE: usize;
}

impl IndexVariable for u8 {
    const MAX_VALUE: usize = 255;
}
impl IndexVariable for u16 {
    const MAX_VALUE: usize = 65535;
}
impl IndexVariable for u32 {
    const MAX_VALUE: usize = 4294967295;
}

pub(crate) struct KeyVector<T: Sized, I: IndexVariable, const N: usize> {
    length: usize,
    capacity: usize,
    indices: [I; N],
    data: [T; N],
}

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
        //        let returned_ptr = unsafe { self.wasm_allocator.internal_alloc(1) };
        //        console_log!("Returned ptr: {:?}", returned_ptr);
    }

    // Until Rust permits 'Placement New' logic, we have to initialize the KeyVector by directly
    // writing bytes into the backing pool.
    // Once Placement New is possible, we can ideally separate the KeyVector back into its own
    // module (we currently need it placed here to access the private fields - so we can write
    // bytes to these private fields).
    pub(super) fn addkeyvec<T: Sized, I: IndexVariable, const N: usize>(&mut self) {
        let tracking_ptr_usize = self.wasm_allocator.tracking_ptr as usize;

        // This check fills the role of a runtime assert that N != 0 which ideally would be placed
        // as a 'static_assert' like in C++.
        // It is possible that we can use const generics to handle these checks at compile time
        // once it stabilizes.
        if N == 0 {
            console_log!("[KeyVector::new()] ERROR: N == 0");
            panic!();
        }

        // This check fills the role of a runtime assert that N <= I::MAX_VALUE + 1 which ideally would be placed
        // as a 'static_assert' like in C++.
        // It is possible that we can use const generics to handle these checks at compile time
        // once it stabilizes.
        if N > (I::MAX_VALUE + 1) {
            console_log!("[KeyVector::new()] ERROR: N > Index::MAX_VALUE");
            panic!();
        }

        // This panic is avoidable by adding padded bytes necessary to obtain a suitable aligned
        // pointer
        if (tracking_ptr_usize % align_of::<KeyVector<T, I, N>>()) != 0 {
            console_log!("[WebCore::addkeyvec()] ERROR: Found unaligned tracking pointer");
            panic!();
        }

        // At this stage:
        // tracking_ptr is suitably aligned to be converted into *mut KeyVector<T, I, N>
        let tracking_ptr = self.wasm_allocator.tracking_ptr as *mut KeyVector<T, I, N>;

        // Because placement new is not available, we initialize the usize 'length' as all zeroes
        unsafe {
            addr_of_mut!((*tracking_ptr).length).write_bytes(0, 1);
        }
    }

    pub(super) fn start_frame() {}

    pub(super) fn end_frame() {}
}
