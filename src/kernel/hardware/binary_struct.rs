use core::{
    cmp::PartialOrd,
    ops::{BitAnd, BitOr, Shl},
};

pub type Byte = BinaryStruct<u8>;
#[repr(C)]
pub struct BinaryStruct<T>(T);
impl<
        T: BinaryOperations
            + Shl<Output = T>
            + PartialOrd
            + BitAnd<Output = T>
            + BitOr<Output = T>
            + Copy,
    > BinaryStruct<T>
{
    pub fn is_set(&self, index: T) -> bool {
        if index < T::bit_size() {
            return self.0 & T::one() << index != T::zero();
        }
        false
    }
    pub fn set(&mut self, index: T) {
        if index < T::bit_size() {
            self.0 = self.0 | T::one() << index;
        }
    }
    pub fn write(&mut self, data: T) {
        self.0 = data;
    }
}

pub trait BinaryOperations {
    fn bit_size() -> Self;
    fn one() -> Self;
    fn zero() -> Self;
}

impl BinaryOperations for u8 {
    fn bit_size() -> Self {
        8
    }

    fn one() -> Self {
        1
    }

    fn zero() -> Self {
        0
    }
}
