use core::fmt::Debug;

use std::mem::{align_of, size_of};
use std::ptr::addr_of_mut;

use super::{console_log, log};
use crate::wasm_allocator::WasmAllocator;

trait UnsignedType: Copy + Debug {
    const MAX_VALUE: usize;
}

impl UnsignedType for u8 {
    const MAX_VALUE: usize = 255;
}

impl UnsignedType for u16 {
    const MAX_VALUE: usize = 65535;
}

impl UnsignedType for u32 {
    const MAX_VALUE: usize = 4294967295;
}

trait IndexType: PartialEq<i32> {}

struct Index<T: UnsignedType>(T);

impl IndexType for Index<u8> {}
impl IndexType for Index<u16> {}
impl IndexType for Index<u32> {}

impl PartialEq<i32> for Index<u8> {
    fn eq(&self, other: &i32) -> bool {
        // Safe to 'upcast' a u8 to i32 because no loss of bit information
        if (self.0 as i32) == *other {
            return true;
        } else {
            return false;
        }
    }
}

impl PartialEq<i32> for Index<u16> {
    fn eq(&self, other: &i32) -> bool {
        // Safe to 'upcast' a u16 to i32 because no loss of bit information
        if (self.0 as i32) == *other {
            return true;
        } else {
            return false;
        }
    }
}

impl PartialEq<i32> for Index<u32> {
    fn eq(&self, other: &i32) -> bool {
        // When the i32 value 'other' is negative, it cannot equal a u32.
        // Handle this condition first.
        if *other < 0 {
            return false;
        }

        // When the i32 value 'other' is positive, or zero, we can safely 'upcast' it to a u32.
        if self.0 == (*other as u32) {
            return true;
        } else {
            return false;
        }
    }
}

pub(crate) struct KeyVector<T: Sized, I: UnsignedType, const N: usize> {
    length: usize,
    capacity: usize,
    indices: [Index<I>; N],
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

    // Until Rust permits 'Placement New' logic, we have to initialize the KeyVector by directly
    // writing bytes into the backing pool.
    // Once Placement New is possible, we can ideally separate the KeyVector back into its own
    // module (we currently need it placed here to access the private fields - so we can write
    // bytes to these private fields).
    pub(super) fn addkeyvec<T: Sized, I: UnsignedType, const N: usize>(&mut self)
    where
        Index<I>: PartialEq<i32>,
    {
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

        // This panic is avoidable. By adding padded bytes necessary to obtain a suitable aligned
        // pointer
        if (tracking_ptr_usize % align_of::<KeyVector<T, I, N>>()) != 0 {
            console_log!("[WebCore::addkeyvec()] ERROR: Found unaligned tracking pointer");
            panic!();
        }

        if self.wasm_allocator.allocation_size < size_of::<KeyVector<T, I, N>>() {
            console_log!("[WebCore::addkeyvec()] ERROR: Allocation not large enough!");
            console_log!(
                "[WebCore::addkeyvec()] Allocation Size: {}",
                self.wasm_allocator.allocation_size
            );
            console_log!(
                "[WebCore::addkeyvec()] KeyVector Size: {}",
                size_of::<KeyVector<T, I, N>>()
            );
            panic!();
        }

        // At this stage:
        // tracking_ptr is suitably aligned to be converted into *mut KeyVector<T, I, N>
        let tracking_ptr = self.wasm_allocator.tracking_ptr as *mut KeyVector<T, I, N>;

        // Because placement new is not available, we initialize the field addresses of
        // the first three variables (length, capacity, and indices set to ZERO.)

        unsafe {
            addr_of_mut!((*tracking_ptr).length).write_bytes(0, 1);
            addr_of_mut!((*tracking_ptr).capacity).write_bytes(0, 1);

            // WE *MUST* CONFIRM THIS ZEROS THE ENTIRE ARRAY!!!
            addr_of_mut!((*tracking_ptr).indices).write_bytes(0, 1);

            assert!(
                size_of::<[I; N]>() == (N * size_of::<I>()),
                "Array sizing check"
            );

            // This confirms that all values within the array [Index<I>; N] are properly ZEROED.
            // This is ONLY FOR TESTING PURPOSES, ensures that the code above:
            //
            // addr_of_mut!((*tracking_ptr).indices).write_bytes(0, 1);
            //
            // ... has indeed cleared the entire array.
            for i in 0..(*tracking_ptr).indices.len() {
                if (*tracking_ptr).indices[i] != 0 {
                    console_log!(
                        "Invalid zeroing!!! I: {:?}, Value: {:?}",
                        i,
                        (*tracking_ptr).indices[i].0
                    );
                    panic!();
                }
            }
        }
    }
}
