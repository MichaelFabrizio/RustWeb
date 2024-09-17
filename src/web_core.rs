use std::convert::TryFrom;
use std::mem::{align_of, size_of};
use std::ptr::{addr_of_mut, read, write};

use super::{console_log, log};
use crate::indexing::{Index, IndexType, UnsignedType};
use crate::wasm_allocator::WasmAllocator;

pub(crate) struct KeyVector<T: Sized, I: UnsignedType, const N: usize> {
    length: usize,
    indices: [Index<I>; N],
    data: [T; N],
}

impl<T: Sized, I: UnsignedType, const N: usize> KeyVector<T, I, N> {
    pub(crate) fn add(&mut self, key: usize)
    where
        T: Default,
        Index<I>: IndexType,
    {
        // Invalid key bounds
        if key == 0 || key >= N {
            return;
        }

        if key == (self.length + 1) {
            self.add_equal_key_checked(key);
            return;
        }
    }

    pub(crate) fn find(&self, key: usize) -> &T
    where
        T: Default,
        Index<I>: IndexType,
    {
        if key >= N {
            return &self.data[0];
        }

        if key <= self.length {
            if key == self.indices[key].into() {
                return &self.data[key];
            }
            return &self.data[0];
        }

        let key_location: usize = self.indices[key].into();
        return &self.data[key_location];
    }

    pub(crate) fn find_mut(&mut self, key: usize) -> &mut T
    where
        T: Default,
        Index<I>: IndexType,
    {
        if key >= N {
            return &mut self.data[0];
        }

        if key <= self.length {
            if key == self.indices[key].into() {
                return &mut self.data[key];
            }
            return &mut self.data[0];
        }

        let key_location: usize = self.indices[key].into();
        return &mut self.data[key_location];
    }

    fn add_equal_key_checked(&mut self, key: usize)
    where
        T: Default,
        Index<I>: IndexType,
    {
        if self.indices[key] != 0 {
            return;
        }

        let index_key = Self::usize_to_index(key);
        self.indices[key] = index_key;

        let src: T = Default::default();
        let dst = addr_of_mut!(self.data[key]);
        unsafe {
            write(dst, src);
        }

        self.length += 1;
    }

    fn add_greater_key_checked(&mut self, key: usize)
    where
        T: Default,
        Index<I>: IndexType,
    {
        if self.indices[key] != 0 {
            return;
        }

        let index_pointer: usize = self.indices[self.length + 1].into();

        // BRANCH POSSIBILITY #1
        // CONDITION: self.indices[self.length + 1] != 0
        // *(Requires extra swapping logic)*

        if index_pointer != 0 {
            self.indices[self.length + 1] = Self::usize_to_index(self.length + 1);
            self.indices[index_pointer] = Self::usize_to_index(key);
            self.indices[key] = Self::usize_to_index(index_pointer);

            unsafe {
                let src = read(&self.data[index_pointer]);
                let dst = addr_of_mut!(self.data[self.length + 1]);
                write(dst, src);

                let src: T = Default::default();
                let dst = addr_of_mut!(self.data[index_pointer]);
                write(dst, src);
            }
            self.length += 1;
            return;
        }

        // BRANCH POSSIBILITY #2
        // CONDITION: self.indices[self.length + 1] == 0
        // *(Free to place greater key directly at self.length + 1)*

        self.indices[self.length + 1] = Self::usize_to_index(key);
        self.indices[key] = Self::usize_to_index(self.length + 1);

        unsafe {
            let src: T = Default::default();
            let dst = addr_of_mut!(self.data[self.length + 1]);
            write(dst, src);
        }
        self.length += 1;
    }

    fn usize_to_index(key: usize) -> Index<I>
    where
        Index<I>: IndexType,
    {
        let downcast_result = Index::<I>::try_from(key);

        // Match statement below can be removed once #[derive(Debug)] is correctly implemented for
        // enums.
        //
        // Replace above with:
        // let result = Index::<I>::try_from(key).expect("Error: {:?}");

        if let Ok(valid_index) = downcast_result {
            // TODO: Remove console log
            console_log!("Success: {:?}", valid_index);
            return valid_index;
        } else {
            console_log!("Error: Usize bad downcast");
            panic!();
        }
    }
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
    pub(super) fn addkeyvec<T: Sized, I: UnsignedType, const N: usize>(
        &mut self,
    ) -> *mut KeyVector<T, I, N>
    where
        T: Default,
        Index<I>: IndexType,
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
        let casted_ptr = self.wasm_allocator.tracking_ptr as *mut KeyVector<T, I, N>;

        // Because placement new is not available, we initialize the field addresses of
        // the first two variables (length, and indices set to ZERO.)

        unsafe {
            addr_of_mut!((*casted_ptr).length).write_bytes(0, 1);
            // WE *MUST* CONFIRM THIS ZEROS THE ENTIRE ARRAY!!!
            addr_of_mut!((*casted_ptr).indices).write_bytes(0, 1);

            // This confirms that all values within the array [Index<I>; N] are cleared to zero.
            // The entire array [Index<I>; N] is cycled
            // Each value is tested against zero.
            //
            for i in 0..(*casted_ptr).indices.len() {
                if (*casted_ptr).indices[i] != 0 {
                    console_log!(
                        "Invalid zeroing!!! I: {:?}, Value: {:?}",
                        i,
                        (*casted_ptr).indices[i].0
                    );
                    panic!();
                }
            }

            // Set zero element to a default value
            let src: T = Default::default();
            let dst = addr_of_mut!((*casted_ptr).data[0]);
            std::ptr::write(dst, src);
        }

        // TODO: Subtract byte size from wasm_allocator.allocation_size

        // TODO: Move tracking pointer by byte size

        // TODO: Return *mut KeyVector<T, I, N> (or consider RefCell or something?)
        return casted_ptr;
    }
}
