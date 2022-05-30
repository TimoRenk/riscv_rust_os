use core::{
    cmp::PartialOrd,
    ops::{BitAnd, BitOr, Range, Shl},
};

pub type Byte = BinaryStruct<u8>;
#[repr(C)]
pub struct BinaryStruct<T>(T);
impl<T> BinaryStruct<T>
where
    T: BinaryOperations
        + Shl<Output = T>
        + PartialOrd
        + BitAnd<Output = T>
        + BitOr<Output = T>
        + Copy,
    Range<T>: Iterator<Item = T>,
{
    pub fn is_set(&self, index: usize) -> bool {
        if let Some(index) = Self::convert_index(index) {
            return self.0 & T::one() << index != T::zero();
        }
        return false;
    }
    pub fn at(&mut self, index: usize, set: bool) {
        let index = match Self::convert_index(index) {
            Some(index) => index,
            None => return,
        };
        if set {
            self.0 = self.0 | T::one() << index;
        } else {
            self.0 = self.0 & (T::one() << index).inverse();
        }
    }
    pub fn write_register_entry(&mut self, register_entry: RegisterEntry) {
        let (index, set) = register_entry;
        self.at(index, set)
    }
    pub fn write(&mut self, data: T) {
        self.0 = data;
    }
    pub fn get(&self) -> T {
        self.0
    }
    fn convert_index(index: usize) -> Option<T> {
        if index >= T::bit_size() {
            return None;
        }
        Some(T::from(index))
    }
}
impl<T> From<T> for BinaryStruct<T> {
    fn from(data: T) -> Self {
        BinaryStruct(data)
    }
}

pub trait BinaryOperations {
    fn bit_size() -> usize;
    fn one() -> Self;
    fn zero() -> Self;
    fn inverse(self) -> Self;
    fn from(data: usize) -> Self;
}

impl BinaryOperations for u8 {
    fn bit_size() -> usize {
        8
    }
    fn one() -> Self {
        1
    }
    fn zero() -> Self {
        0
    }
    fn inverse(self) -> Self {
        !self
    }
    fn from(data: usize) -> Self {
        data as Self
    }
}
impl BinaryOperations for u64 {
    fn bit_size() -> usize {
        64
    }
    fn one() -> Self {
        1
    }
    fn zero() -> Self {
        0
    }
    fn inverse(self) -> Self {
        !self
    }

    fn from(data: usize) -> Self {
        data as Self
    }
}
pub type RegisterEntry = (usize, bool);
