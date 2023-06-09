use core::fmt::Debug;
use core::ops::{Div, Rem};

use crate::scheduler::Prog;

use super::binary_struct::{BinaryStruct, Byte, MaxDigits};
use super::memory_mapping::MemoryMapping;
use super::ring_buffer::RingBuffer;
use super::sync::Protected;

const BASE_ADDR: usize = 0x1000_0000;

static READ_CHAR: Protected<RingBuffer<char>> = Protected::new(RingBuffer::new('X'));

pub static UART: Protected<Uart> = Protected::new(Uart::new());

pub fn init() {
    unsafe {
        let mem_ier = &UART.lock().reg.ier_dlm;
        let mut ier = BinaryStruct::from(0);
        ier.at(0, true); // receive interrupt
        ier.at(1, false); // transmit interrupt
        ier.at(2, false); // receiver line status interrupt
        ier.at(3, false); // receiver transmit status interrupt
        mem_ier.write(ier);
    }
}

pub fn get_interrupt_cause() -> Interrupt {
    unsafe {
        let isr = UART.lock().reg.isr_fcr.read();
        let b0 = isr.is_set(0);
        let b1 = isr.is_set(1);
        let b2 = isr.is_set(2);
        let b3 = isr.is_set(3);
        if b0 {
            return Interrupt::Error;
        }
        if b1 && b2 && !b3 {
            return Interrupt::LineStatusReg;
        }
        if !b1 && b2 && !b3 {
            return Interrupt::ReceivedDataRdy;
        }
        if !b1 && b2 && b3 {
            return Interrupt::ReceivedDataTimeout;
        }
        if b1 && !b2 && !b3 {
            return Interrupt::TransHoldRegEmpty;
        }
        if !b1 && !b2 && !b3 {
            return Interrupt::ModemStatusReg;
        }
        Interrupt::Error
    }
}
/// Only call if an interrupt happened.
/// Returns the blocking user prog.
pub unsafe fn read_char_to_buffer() -> Option<Prog> {
    let uart = UART.lock();
    let char = uart.read_char();
    let open_user_prog = uart.open_user_prog;
    uart.unlock();
    if open_user_prog.is_some() {
        READ_CHAR.lock().write(char);
    }
    open_user_prog
}
pub fn get_char() -> Option<char> {
    let char = READ_CHAR.lock().read();
    char
}
pub fn print_char(char: char) {
    UART.lock().print_char(char as u8);
}
/// Opens 'read' and returns true if successful or already blocked by the user_prog.
/// False if blocked by a different user_prog.
pub fn open(user_prog: Prog) -> bool {
    let mut uart = UART.lock();
    if let Some(open) = uart.open_user_prog {
        return open == user_prog;
    }
    uart.open_user_prog = Some(user_prog);
    READ_CHAR.lock().clear();
    true
}

/// Closes 'read' if it is blocked by the user_prog. Returns true when successful.
pub fn close(user_prog: Prog) -> bool {
    let mut uart = UART.lock();
    if Some(user_prog) == uart.open_user_prog {
        uart.open_user_prog = None;
        return true;
    }
    false
}

pub fn is_open(user_prog: Prog) -> bool {
    UART.lock().open_user_prog == Some(user_prog)
}

pub fn print_num<T, const DIGITS: usize>(number: T)
where
    T: From<u8>
        + MaxDigits<DIGITS>
        + PartialEq
        + Rem<Output = T>
        + Div<Output = T>
        + Copy
        + TryInto<u8>,
{
    let digits = to_single_digits(number);
    let mut uart = UART.lock();
    let mut first = false;
    for byte in digits {
        if byte != 0 {
            first = true;
        }
        if first {
            let ascii = byte + 0x30;
            uart.print_char(ascii);
        }
    }
    if !first {
        let ascii = 0x30;
        uart.print_char(ascii);
    }
}

fn to_single_digits<T, const DIGITS: usize>(number: T) -> [u8; DIGITS]
where
    T: From<u8>
        + MaxDigits<DIGITS>
        + PartialEq
        + Rem<Output = T>
        + Div<Output = T>
        + Copy
        + TryInto<u8>,
{
    let mut digits = T::max_digits();
    let mut number = number;
    let mut index = digits.len() - 1;
    while number != T::from(0) {
        digits[index] = (number % T::from(10))
            .try_into()
            .unwrap_or_else(|_| panic!("to_single_digits should always fit into u8!"));
        number = number / T::from(10);
        index -= 1;
    }
    digits
}

pub struct Uart {
    reg: UartRegister,
    open_user_prog: Option<Prog>,
}
impl Uart {
    const fn new() -> Self {
        Uart {
            reg: UartRegister::new(BASE_ADDR),
            open_user_prog: None,
        }
    }
    fn print_char(&mut self, char: u8) {
        unsafe {
            loop {
                let lsr = self.reg.lsr.read();
                if lsr.is_set(5) {
                    self.reg.rbr_thr_dll.write(char);
                    return;
                }
            }
        }
    }

    fn read_char(&self) -> char {
        unsafe { self.reg.rbr_thr_dll.read() as char }
    }
}

impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            self.print_char(c as u8);
        }
        Ok(())
    }
}

#[allow(dead_code)]
struct UartRegister {
    /// Receive Buffer Register, Transmit Holding Register | LSB of Divisor Latch when enabled.
    rbr_thr_dll: MemoryMapping<u8>,
    /// N/A, Interrupt Enable Register | MSB of Divisor Latch when enabled.
    ier_dlm: MemoryMapping<Byte>,
    /// Interrupt Status Register, FIFO control Register
    isr_fcr: MemoryMapping<Byte>,
    /// N/A, Line Control Register
    lcr: MemoryMapping<Byte>,
    /// N/A, Modem Control Register
    mcr: MemoryMapping<Byte>,
    /// Line Status Register, N/A
    lsr: MemoryMapping<Byte>,
    /// Modem Status Register, N/A
    msr: MemoryMapping<Byte>,
    /// Scratchpad Register Read, Scratchpad Register Write
    scr: MemoryMapping<Byte>,
}
impl UartRegister {
    const fn new(addr: usize) -> Self {
        UartRegister {
            rbr_thr_dll: MemoryMapping::new(addr),
            ier_dlm: MemoryMapping::new(addr + 1),
            isr_fcr: MemoryMapping::new(addr + 2),
            lcr: MemoryMapping::new(addr + 3),
            mcr: MemoryMapping::new(addr + 4),
            lsr: MemoryMapping::new(addr + 5),
            msr: MemoryMapping::new(addr + 6),
            scr: MemoryMapping::new(addr + 7),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Interrupt {
    LineStatusReg,
    ReceivedDataRdy,
    ReceivedDataTimeout,
    TransHoldRegEmpty,
    ModemStatusReg,
    Error,
}
