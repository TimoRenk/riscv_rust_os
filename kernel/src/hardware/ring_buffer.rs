use core::fmt::Display;

use crate::macros::print;

/// Actual size is 1 less due to the head and tail pointer.
pub const BUFFER_SIZE: usize = 21;

pub const fn new<T>(buffer: [T; BUFFER_SIZE]) -> RingBuffer<T> {
    RingBuffer {
        buffer,
        head: 0,
        tail: 0,
    }
}

pub struct RingBuffer<T> {
    buffer: [T; BUFFER_SIZE],
    head: usize,
    tail: usize,
}
impl<T> RingBuffer<T>
where
    T: Display + Copy,
{
    pub fn read(&mut self) -> Option<T> {
        if self.head != self.tail {
            let val = Some(self.buffer[self.tail]);
            Self::increase_idx(&mut self.tail);
            return val;
        }
        return None;
    }
    pub fn write(&mut self, val: T) {
        self.buffer[self.head] = val;
        Self::increase_idx(&mut self.head);
        if self.head == self.tail {
            unsafe {
                print!(
                    "\nRing buffer full, last element lost: {}!\n",
                    self.buffer[self.tail]
                );
            }
            Self::increase_idx(&mut self.tail);
        }
    }
    pub fn clear(&mut self) {
        self.tail = self.head;
    }
    fn increase_idx(cur_idx: &mut usize) {
        *cur_idx = (*cur_idx + 1) % BUFFER_SIZE;
    }
}
