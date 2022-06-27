pub use core::arch::asm;

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
        let program_ptr = 0x80100000u64;
        riscv_utils::write_machine_reg!(program_ptr => "mepc");
        asm!("mret");
    }
}
