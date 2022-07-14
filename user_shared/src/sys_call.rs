#![allow(dead_code)]
use core::arch::asm;
use riscv_utils as riscv;
use riscv_utils::SysCall;

unsafe fn system_call(syscall: SysCall, param_0: u64, param_1: u64) -> u64 {
    let number = syscall as u64;
    riscv::write_function_reg!(
        number => "a7",
        param_0 => "a0",
        param_1 => "a1"
    );
    asm!("ecall");
    let output;
    riscv::read_function_reg!("a0" => output);
    output
}

pub fn print_char(char: char) {
    unsafe {
        system_call(SysCall::PrintChar, char as u64, 0);
    }
}

/// Requires uart to be open. Returns 'None' otherwise.
pub fn get_char() -> Option<char> {
    unsafe {
        let res = system_call(SysCall::GetChar, 0, 0);
        if res == 0 {
            return None;
        }
        return Some(res as u8 as char);
    }
}

pub fn print(string: &str) {
    if string.is_empty() {
        return;
    }
    unsafe {
        system_call(
            SysCall::PrintString,
            string.as_ptr() as u64,
            string.len() as u64,
        );
    }
}
//todo this should be changed to one system call..
pub fn println(string: &str) {
    print(string);
    print_char('\n');
}
pub fn print_num(number: u64) {
    unsafe {
        system_call(SysCall::PrintNum, number, 0);
    }
}
pub fn exit() {
    unsafe {
        system_call(SysCall::Exit, 0, 0);
    }
}
pub fn sys_yield() {
    unsafe {
        system_call(SysCall::Yield, 0, 0);
    }
}
pub fn uart_open() -> bool {
    unsafe {
        return system_call(SysCall::UartOpen, 0, 0) != 0;
    }
}
pub fn uart_close() -> bool {
    unsafe {
        return system_call(SysCall::UartClose, 0, 0) != 0;
    }
}
