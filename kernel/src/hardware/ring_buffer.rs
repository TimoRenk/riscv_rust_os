//! A simple ring buffer.

use core::fmt::{Debug, Display};

use crate::macros::print;

pub const BUFFER_SIZE: usize = 10;

pub struct RingBuffer<T> {
    buffer: [T; BUFFER_SIZE],
    head: usize,
    tail: usize,
    empty: bool,
}
impl<T> RingBuffer<T>
where
    T: Display + Debug + Copy + Default + PartialEq<char>,
{
    pub const fn new(default: T) -> Self {
        RingBuffer {
            buffer: [default; BUFFER_SIZE],
            head: 0,
            tail: 0,
            empty: true,
        }
    }
    pub fn read(&mut self) -> Option<T> {
        if self.empty {
            return None;
        }
        let val = Some(self.buffer[self.tail]);
        Self::increase_idx(&mut self.tail);
        if self.tail == self.head {
            self.empty = true;
        }
        val
    }
    pub fn write(&mut self, val: T) {
        let previous = self.buffer[self.head];
        self.buffer[self.head] = val;
        if !self.empty && self.head == self.tail {
            print!(
                "\nRing buffer full, last element lost: {}\nRing buffer: {:?}\n",
                previous, self.buffer
            );
            Self::increase_idx(&mut self.tail);
        }
        self.empty = false;
        Self::increase_idx(&mut self.head);
    }
    pub fn clear(&mut self) {
        self.tail = self.head;
        self.empty = true;
    }
    fn increase_idx(cur_idx: &mut usize) {
        *cur_idx = (*cur_idx + 1) % BUFFER_SIZE;
    }
}
