use super::{console_log, log};
use std::mem::MaybeUninit;

pub(crate) trait IndexVariable {
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
    indices: [I; N],
    data: [MaybeUninit<T>; N],
}

impl<T: Sized, I: IndexVariable, const N: usize> KeyVector<T, I, N> {
    // Until const generic evaluation is stabilized, better to just panic!
    // https://rust-lang.github.io/rfcs/2000-const-generics.html
    //
    // Note: panic!() leads to "RuntimeError: unreachable executed" which isn't really
    // debuggable. Consider this crate: https://rustwasm.github.io/docs/book/reference/debugging.html#logging-panics
    pub(crate) fn new() {
        if N == 0 {
            console_log!("[KeyVector::new()] ERROR: N == 0");
            panic!();
        }
        if N > (I::MAX_VALUE + 1) {
            console_log!("[KeyVector::new()] ERROR: N > Index::MAX_VALUE");
            panic!();
        }
    }
}
