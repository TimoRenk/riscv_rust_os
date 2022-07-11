use core::ops::{Div, Rem};

use super::binary_struct::{BinaryOperations, Byte, MaxDigits};
use super::memory_mapping::MemoryMapping;

const BASE_ADDR: usize = 0x1000_0000;

static mut UART: UART = UART {
    reg: UartRegister::new(BASE_ADDR),
};

//todo catch race condition?!
pub unsafe fn print_str(str: &str) {
    str.chars().for_each(|c| print_char(c));
}
pub unsafe fn print_char(char: char) {
    UART.print_char(char as u8);
}
pub unsafe fn get_char() -> char {
    UART.get_char()
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
}

impl UART {
    fn print_char(&mut self, char: u8) {
        unsafe {
            loop {
                let lsr = self.reg.5.read();
                if lsr.is_set(5) {
                    self.reg.0.write(char);
                    return;
                }
            }
        }
    }

    fn get_char(&self) -> char {
        unsafe {
            loop {
                let lsr = self.reg.5.read();
                if lsr.is_set(0) {
                    return self.reg.0.read() as char;
                }
            }
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

/// offset_0: RBR, THR | DLL
/// offset_1: IER | DLM
/// offset_2: IIR, FCR
/// offset_3: LCR
/// offset_4: MCR
/// offset_5: LSR
/// offset_6: MSR
/// offset_7: SCR
struct UartRegister(
    MemoryMapping<u8>,
    MemoryMapping<Byte>,
    MemoryMapping<Byte>,
    MemoryMapping<Byte>,
    MemoryMapping<Byte>,
    MemoryMapping<Byte>,
    MemoryMapping<Byte>,
    MemoryMapping<Byte>,
);
impl UartRegister {
    const fn new(addr: usize) -> Self {
        let offset_0 = MemoryMapping::new(addr);
        let offset_1 = MemoryMapping::new(addr + 1);
        let offset_2 = MemoryMapping::new(addr + 2);
        let offset_3 = MemoryMapping::new(addr + 3);
        let offset_4 = MemoryMapping::new(addr + 4);
        let offset_5 = MemoryMapping::new(addr + 5);
        let offset_6 = MemoryMapping::new(addr + 6);
        let offset_7 = MemoryMapping::new(addr + 7);
        UartRegister(
            offset_0, offset_1, offset_2, offset_3, offset_4, offset_5, offset_6, offset_7,
        )
    }
}
