use core::{
    cmp::PartialOrd,
    ops::{BitAnd, BitOr, Range, Shl},
};
use riscv_utils::RegisterEntry;

pub type Byte = BinaryStruct<u8>;
#[repr(C)]
#[derive(Clone, Copy)]
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
    pub fn is_set(&self, bit: usize) -> bool {
        if let Some(bit) = Self::transform_bit(bit) {
            return self.0 & T::one() << bit != T::zero();
        }
        return false;
    }
    pub fn at(&mut self, bit: usize, set: bool) {
        let bit = match Self::transform_bit(bit) {
            Some(bit) => bit,
            None => return,
        };
        if set {
            self.0 = self.0 | T::one() << bit;
        } else {
            self.0 = self.0 & (T::one() << bit).inverse();
        }
    }
    pub fn write_register_entry(&mut self, register_entry: RegisterEntry) {
        let (bit, set) = register_entry;
        self.at(bit, set)
    }
    pub fn get(&self) -> T {
        self.0
    }
    fn transform_bit(bit: usize) -> Option<T> {
        if bit >= T::bit_size() {
            return None;
        }
        Some(T::from(bit))
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
    fn ten() -> Self;
    fn inverse(self) -> Self;
    fn from(data: usize) -> Self;
    fn into_u8(self) -> u8;
}

impl BinaryOperations for u8 {
    fn bit_size() -> usize {
        u8::BITS as usize
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

    fn ten() -> Self {
        10
    }

    fn into_u8(self) -> u8 {
        self as u8
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

impl BinaryOperations for u64 {
    fn bit_size() -> usize {
        u64::BITS as usize
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

    fn ten() -> Self {
        10
    }

    fn into_u8(self) -> u8 {
        self as u8
    }
}

impl BinaryOperations for usize {
    fn bit_size() -> usize {
        u64::BITS as usize
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

    fn ten() -> Self {
        10
    }

    fn into_u8(self) -> u8 {
        self as u8
    }
}
impl BinaryOperations for u32 {
    fn bit_size() -> usize {
        u32::BITS as usize
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

    fn ten() -> Self {
        10
    }

    fn into_u8(self) -> u8 {
        self as u8
    }
}
