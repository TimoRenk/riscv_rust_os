//! A binary structure for easy bit manipulation.
//! 
//! The inner type T is the container for the bits.

use core::mem::size_of;
use core::ops::Not;
use core::{
    cmp::PartialOrd,
    ops::{BitAnd, BitOr, Shl},
};
use riscv_utils::RegisterEntry;

pub type Byte = BinaryStruct<u8>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BinaryStruct<T>(T);
impl<T> BinaryStruct<T>
where
    T: Shl<usize, Output = T>
        + PartialOrd
        + BitAnd<Output = T>
        + BitOr<Output = T>
        + Not<Output = T>
        + Copy
        + From<u8>,
{
    const SIZE: usize = size_of::<T>() * 8;
    pub fn is_set(&self, bit: usize) -> bool {
        Self::assert_valid(bit);
        self.0 & T::from(1u8) << bit != T::from(0u8)
    }
    pub fn at(&mut self, bit: usize, set: bool) {
        Self::assert_valid(bit);
        if set {
            self.0 = self.0 | T::from(1u8) << bit;
        } else {
            self.0 = self.0 & !(T::from(1u8) << bit);
        }
    }
    pub fn write_register_entry(&mut self, register_entry: RegisterEntry) {
        let (bit, set) = register_entry;
        self.at(bit, set)
    }
    pub fn into_inner(self) -> T {
        self.0
    }
    /// Checks if the specified bit fits into the bit-size of T.
    fn assert_valid(bit: usize) {
        assert!(
            bit < Self::SIZE,
            "Accessed a binary struct outside its size '{}'. Accessed '{}'",
            Self::SIZE,
            bit
        );
    }
}

impl<T> From<T> for BinaryStruct<T> {
    fn from(data: T) -> Self {
        BinaryStruct(data)
    }
}

pub trait MaxDigits<const DIGITS: usize> {
    fn max_digits() -> [u8; DIGITS];
}

impl MaxDigits<20> for usize {
    fn max_digits() -> [u8; 20] {
        [0; 20]
    }
}
