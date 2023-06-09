//! The user side of the system calls.

#![allow(dead_code)]
use core::arch::asm;
use riscv_utils as riscv;
use riscv_utils::SysCall;

unsafe fn sys_call(syscall: SysCall, param_0: usize, param_1: usize) -> usize {
    let number = syscall as usize;
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
        sys_call(SysCall::PrintChar, char as usize, 0);
    }
}

/// Requires uart to be open. Returns 'None' otherwise.
pub fn get_char() -> Option<char> {
    unsafe {
        let res = sys_call(SysCall::GetChar, 0, 0);
        if res == 0 {
            return None;
        }
        Some(res as u8 as char)
    }
}

pub fn print(string: &str) {
    if string.is_empty() {
        return;
    }
    unsafe {
        sys_call(SysCall::PrintString, string.as_ptr() as usize, string.len());
    }
}

pub fn print_num(number: usize) {
    unsafe {
        sys_call(SysCall::PrintNum, number, 0);
    }
}

pub fn exit() {
    unsafe {
        sys_call(SysCall::Exit, 0, 0);
    }
}

pub fn sys_yield() {
    unsafe {
        sys_call(SysCall::Yield, 0, 0);
    }
}

pub fn uart_open() -> bool {
    unsafe { sys_call(SysCall::UartOpen, 0, 0) != 0 }
}

pub fn uart_close() -> bool {
    unsafe { sys_call(SysCall::UartClose, 0, 0) != 0 }
}
