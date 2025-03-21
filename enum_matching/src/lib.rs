//! Enum Matching is a simple crate which contains a single derive macro -- **EnumTryFrom**.
//!
//! This derive macro adds a [TryFrom] implementation for enums to create an enum variant from an [isize].

#![no_std]
use core::cmp::{Eq, PartialEq};
use core::fmt::Debug;
use core::fmt::Display;
use core::prelude::rust_2021::derive;
use core::write;

pub use enum_matching_derive::EnumTryFrom;

#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    pub num: isize,
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "No discriminant is matching number: {}", self.num)
    }
}
