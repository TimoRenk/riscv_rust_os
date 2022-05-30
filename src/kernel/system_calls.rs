use super::hardware::riscv::*;
use core::arch::asm;

pub const PRINT_CHAR: u64 = 1;
pub const PRINT_STRING: u64 = 2;

fn system_call(number: u64, param_1: u64, param_2: u64) -> u64 {
    let output;
    unsafe {
        asm!("add a7, {}, zero", in(reg) number);
        asm!("add a1, {}, zero", in(reg) param_2);
        asm!("add a0, {}, zero", in(reg) param_1);
        asm!("ecall");
        asm!("add {}, a0, zero", out(reg) output);
    }

    output
}
#[allow(dead_code)]
pub fn print_char(char: char) {
    system_call(PRINT_CHAR, char as u64, 0);
}
#[allow(dead_code)]
pub fn print_string(string: &str) {
    system_call(PRINT_STRING, string.as_ptr() as u64, string.len() as u64);
}
