use core::convert::TryFrom;
use core::fmt::Debug;

pub(crate) trait UnsignedType: Copy + Debug {
    const MAX_VALUE: usize;
}

pub(crate) trait IndexType: Copy + PartialEq<i32> + TryFrom<usize> + Into<usize> {}

impl UnsignedType for u8 {
    const MAX_VALUE: usize = 255;
}

impl UnsignedType for u16 {
    const MAX_VALUE: usize = 65_535;
}

impl UnsignedType for u32 {
    const MAX_VALUE: usize = 1_000_000;
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct Index<T: UnsignedType>(pub(crate) T);

#[derive(Debug)]
pub(crate) enum IndexError {
    UsizeDowncastError,
}

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

impl TryFrom<usize> for Index<u8> {
    type Error = IndexError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value > u8::MAX_VALUE {
            Err(IndexError::UsizeDowncastError)
        } else {
            Ok(Index(value as u8))
        }
    }
}

impl TryFrom<usize> for Index<u16> {
    type Error = IndexError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value > u16::MAX_VALUE {
            Err(IndexError::UsizeDowncastError)
        } else {
            Ok(Index(value as u16))
        }
    }
}

impl TryFrom<usize> for Index<u32> {
    type Error = IndexError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value > u32::MAX_VALUE {
            Err(IndexError::UsizeDowncastError)
        } else {
            Ok(Index(value as u32))
        }
    }
}

impl From<Index<u8>> for usize {
    fn from(value: Index<u8>) -> Self {
        value.0 as usize
    }
}

impl From<Index<u16>> for usize {
    fn from(value: Index<u16>) -> Self {
        value.0 as usize
    }
}

impl From<Index<u32>> for usize {
    fn from(value: Index<u32>) -> Self {
        value.0 as usize
    }
}
