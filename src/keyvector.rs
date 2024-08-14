use std::mem::MaybeUninit;

pub(crate) struct KeyVector<T, I, const N: usize> {
    indices: [I; N],
    data: [MaybeUninit<T>; N],
}

impl<T, I, const N: usize> KeyVector<T, I, N> {}
