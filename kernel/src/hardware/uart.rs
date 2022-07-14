use core::ops::{Div, Rem};

use crate::scheduler::Prog;

use super::binary_struct::{BinaryOperations, BinaryStruct, Byte, MaxDigits};
use super::memory_mapping::MemoryMapping;
use super::ring_buffer::{self, RingBuffer, BUFFER_SIZE};

const BASE_ADDR: usize = 0x1000_0000;

static mut READ_CHAR: RingBuffer<char> = ring_buffer::new(['X'; BUFFER_SIZE]);

static mut UART: UART = UART {
    reg: UartRegister::new(BASE_ADDR),
    open_user_prog: None,
};

pub unsafe fn init() {
    let mem_ier = &mut UART.reg.ier_dlm;
    let mut ier = BinaryStruct::from(0);
    ier.at(0, true); // receive interrupt
    ier.at(1, false); // transmit interrupt
    ier.at(2, false); // receiver line status interrupt
    ier.at(3, false); // receiver transmit status interrupt
    mem_ier.write(ier);
}

pub unsafe fn get_interrupt_cause() -> Interrupt {
    let isr = UART.reg.isr_fcr.read();
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
    return Interrupt::Error;
}

/// Only call if an interrupt happened. Returns the blocking user prog.
pub unsafe fn read_char() -> Option<Prog> {
    READ_CHAR.write(UART.read_char());
    UART.open_user_prog
}
pub unsafe fn get_char() -> Option<char> {
    let char = READ_CHAR.read();
    return char;
}

//todo catch race condition?!
pub unsafe fn print_str(str: &str) {
    str.chars().for_each(|c| print_char(c));
}
pub unsafe fn print_char(char: char) {
    UART.print_char(char as u8);
}
/// Opens 'read' and returns true if successful or already blocked by the user_prog.
/// False if blocked by a different user_prog.
pub unsafe fn open(user_prog: Prog) -> bool {
    if let Some(open) = UART.open_user_prog {
        return open == user_prog;
    } else {
        UART.open_user_prog = Some(user_prog);
        READ_CHAR.clear();
        return true;
    }
}
/// Closes 'read' if it is blocked by user_prog. Returns true when successful.
pub unsafe fn close(user_prog: Prog) -> bool {
    if Some(user_prog) == UART.open_user_prog {
        UART.open_user_prog = None;
        return true;
    }
    return false;
}
pub unsafe fn is_open(user_prog: Prog) -> bool {
    return UART.open_user_prog == Some(user_prog);
}
pub unsafe fn print_num<T, const DIGITS: usize>(number: T)
where
    T: BinaryOperations + MaxDigits<DIGITS> + PartialEq + Rem<Output = T> + Div<Output = T> + Copy,
{
    let digits = to_single_digits(number);

    let mut first = false;
    for byte in digits {
        if byte != 0 {
            first = true;
        }
        if first {
            let ascii = byte + 0x30;
            UART.print_char(ascii);
        }
    }
    if !first {
        let ascii = 0x30;
        UART.print_char(ascii);
    }
}

pub unsafe fn get_uart() -> &'static mut UART {
    &mut UART
}

fn to_single_digits<T, const DIGITS: usize>(number: T) -> [u8; DIGITS]
where
    T: BinaryOperations + MaxDigits<DIGITS> + PartialEq + Rem<Output = T> + Div<Output = T> + Copy,
{
    let mut digits = T::max_digits();
    let mut number = number;
    let mut index = digits.len() - 1;
    while number != T::zero() {
        digits[index] = (number % T::ten()).into_u8();
        number = number / T::ten();
        index = index - 1;
    }
    return digits;
}

pub struct UART {
    reg: UartRegister,
    open_user_prog: Option<Prog>,
}

impl UART {
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
        unsafe {
            return self.reg.rbr_thr_dll.read() as char;
        }
    }
}

impl core::fmt::Write for UART {
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
        let rhr_thr_dll = MemoryMapping::new(addr);
        let ier_dlm = MemoryMapping::new(addr + 1);
        let isr_fcr = MemoryMapping::new(addr + 2);
        let lcr = MemoryMapping::new(addr + 3);
        let mcr = MemoryMapping::new(addr + 4);
        let lsr = MemoryMapping::new(addr + 5);
        let msr = MemoryMapping::new(addr + 6);
        let scr = MemoryMapping::new(addr + 7);
        UartRegister {
            rbr_thr_dll: rhr_thr_dll,
            ier_dlm,
            isr_fcr,
            lcr,
            mcr,
            lsr,
            msr,
            scr,
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
