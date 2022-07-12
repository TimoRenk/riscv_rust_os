pub use core::arch::asm;
use riscv_utils::*;

use super::hardware::{memory_mapping::MemoryMapping, uart};
use crate::user_prog::{self};

fn syscall_from(number: usize) -> SysCall {
    crate::enum_matching!(
        number: SysCall::PrintString,
        SysCall::PrintChar,
        SysCall::GetChar,
        SysCall::PrintNum,
        SysCall::Yield,
        SysCall::Exit
    );
    panic!("Illegal syscall: {}", number);
}

pub unsafe fn syscall(number: usize, param_0: usize, param_1: usize) -> usize {
    match syscall_from(number) {
        SysCall::PrintString => {
            print_string(param_0, param_1);
            user_prog::increment_mepc();
        }
        SysCall::PrintChar => {
            print_char(param_0);
            user_prog::increment_mepc();
        }
        SysCall::GetChar => {
            let char = get_char() as usize;
            user_prog::increment_mepc();
            return char;
        }
        SysCall::PrintNum => {
            print_num(param_0);
            user_prog::increment_mepc();
        }
        SysCall::Exit => exit(),
        SysCall::Yield => {
            user_prog::increment_mepc();
            sys_yield();
        }
    }
    return 0;
}

unsafe fn print_string(str_ptr: usize, size: usize) {
    let mut str_ptr = str_ptr as *const u8;
    for _ in 0..size {
        let char = MemoryMapping::<u8>::new(str_ptr as usize).read();
        uart::print_char(char as char);
        str_ptr = str_ptr.add(1);
    }
}

unsafe fn print_char(char: usize) {
    uart::print_char(char as u8 as char);
}

unsafe fn get_char() -> char {
    uart::get_char()
}

unsafe fn print_num(number: usize) {
    uart::print_num(number);
}

unsafe fn exit() {
    user_prog::start_prog(user_prog::get());
}

unsafe fn sys_yield() {
    let next = user_prog::next();
    user_prog::switch_or_start(next);
}
