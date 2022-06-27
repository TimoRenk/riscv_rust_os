use core::ops::{Div, Rem};

use super::binary_struct::{BinaryOperations, Byte, MaxDigits};
use super::memory_mapping::MemoryMapping;

//todo catch race condition?!
pub unsafe fn print_string(str: &str) {
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

const UART_BASE_ADDR: usize = 0x1000_0000;
static mut UART: UART = UART {
    register: MemoryMapping::new(UART_BASE_ADDR),
};

struct UART {
    register: MemoryMapping<'static, UartRegister>,
}

impl UART {
    fn print_char(&mut self, char: u8) {
        let register = self.register.get();
        while !register.5.is_set(5) {}
        register.0.write(char);
    }

    fn get_char(&mut self) -> char {
        let register = self.register.get();
        while !register.5.is_set(0) {}
        register.0.get() as char
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
#[repr(C)]
struct UartRegister(Byte, Byte, Byte, Byte, Byte, Byte, Byte, Byte);
