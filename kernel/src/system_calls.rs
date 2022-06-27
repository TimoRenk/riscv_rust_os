pub use core::arch::asm;

use riscv_utils::*;

use super::hardware::{memory_mapping::MemoryMapping, uart};
use crate::user_progs::{self, Progs};

pub enum SystemCall {
    PrintString,
    PrintChar,
    PrintNum,
    GetChar,
    Exit = 42,
}
macro_rules! syscall_matching {
    ($number:ident: $($syscall:expr), +) => {
        $(if $number == $syscall as u64 {
            return Ok($syscall);
        }) +
    };
}
pub struct Error {
    pub message: &'static str,
    pub syscall: u64,
}

impl TryFrom<u64> for SystemCall {
    type Error = Error;
    fn try_from(number: u64) -> Result<Self, Error> {
        syscall_matching!(
            number: Self::PrintString,
            Self::PrintChar,
            Self::GetChar,
            Self::PrintNum,
            Self::Exit
        );
        Err(Error {
            message: "Kernel Error: Illegal syscall ",
            syscall: number,
        })
    }
}

pub unsafe fn syscall(number: u64, param_0: u64, param_1: u64) {
    match SystemCall::try_from(number) {
        Ok(number) => match number {
            SystemCall::PrintString => {
                print_string(param_0, param_1);
                increment_mepc();
            }
            SystemCall::PrintChar => {
                print_char(param_0);
                increment_mepc();
            }
            SystemCall::GetChar => {
                let return_value = get_char() as u64;
                write_function_reg!(return_value => "a0");
                increment_mepc();
            }
            SystemCall::PrintNum => {
                print_num(param_0);
                increment_mepc();
            }
            SystemCall::Exit => exit(),
        },
        Err(error) => {
            uart::print_string(error.message);
            uart::print_num(error.syscall);
        }
    }
}

unsafe fn increment_mepc() {
    let mepc: u64;
    read_machine_reg!("mepc" => mepc);
    write_machine_reg!(mepc + 4 => "mepc");
}

pub unsafe fn print_str(str: &str) {
    for char in str.chars() {
        uart::print_char(char);
    }
}

pub unsafe fn print_string(str_ptr: u64, size: u64) {
    let mut str_ptr = str_ptr as *const u8;
    for _ in 0..size {
        let char = *MemoryMapping::<u8>::new(str_ptr as usize).get();
        uart::print_char(char as char);
        str_ptr = str_ptr.add(1);
    }
}

pub unsafe fn print_char(char: u64) {
    uart::print_char(char as u8 as char);
}

pub unsafe fn get_char() -> char {
    uart::get_char()
}

pub unsafe fn print_num(number: u64) {
    uart::print_num(number);
}
static mut USER_PROG: Progs = Progs::User1;
pub fn exit() {
    unsafe {
        match USER_PROG {
            Progs::User1 => {
                user_progs::switch_prog(Progs::User2);
                USER_PROG = Progs::User2;
            }
            Progs::User2 => {
                user_progs::switch_prog(Progs::User1);
                USER_PROG = Progs::User1;
            }
        }
    }
}
