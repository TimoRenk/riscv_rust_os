pub use core::arch::asm;
use riscv_utils::*;

use super::hardware::{memory_mapping::MemoryMapping, uart};
use crate::scheduler::{self};

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
            scheduler::cur().increment_mepc();
        }
        SysCall::PrintChar => {
            uart::print_char(param_0 as u8 as char);
            scheduler::cur().increment_mepc();
        }
        SysCall::GetChar => {
            let char = get_char() as usize;
            scheduler::cur().increment_mepc();
            return char;
        }
        SysCall::PrintNum => {
            uart::print_num(param_0);
            scheduler::cur().increment_mepc();
        }
        SysCall::Exit => exit(),
        SysCall::Yield => {
            scheduler::cur().increment_mepc();
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
    let cur = scheduler::cur();
    let prog_info = cur.prog_info();
    scheduler::end_prog(scheduler::cur());
    scheduler::init_prog(prog_info);
    sys_yield();
}

unsafe fn sys_yield() {
    let next =
        scheduler::next().expect("No next user prog for system yield. Idle task not implemented");
    scheduler::switch(next);
}
