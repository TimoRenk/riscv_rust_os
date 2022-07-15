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
        SysCall::Exit,
        SysCall::UartOpen,
        SysCall::UartClose
    );
    panic!("Illegal syscall: {}", number);
}

pub unsafe fn syscall(number: usize, param_0: usize, param_1: usize) -> Option<usize> {
    match syscall_from(number) {
        SysCall::PrintString => {
            print_string(param_0, param_1);
            scheduler::cur().increment_mepc();
            return None;
        }
        SysCall::PrintChar => {
            uart::print_char(param_0 as u8 as char);
            scheduler::cur().increment_mepc();
            return None;
        }
        SysCall::GetChar => {
            scheduler::cur().increment_mepc();
            return get_char();
        }
        SysCall::PrintNum => {
            uart::print_num(param_0);
            scheduler::cur().increment_mepc();
            return None;
        }
        SysCall::Exit => {
            exit();
            return None;
        }
        SysCall::Yield => {
            scheduler::cur().increment_mepc();
            sys_yield();
            return None;
        }
        SysCall::UartOpen => {
            let open = uart::open(scheduler::cur());
            scheduler::cur().increment_mepc();
            return Some(open as usize);
        }
        SysCall::UartClose => {
            let close = uart::close(scheduler::cur());
            scheduler::cur().increment_mepc();
            return Some(close as usize);
        }
    }
}

unsafe fn print_string(str_ptr: usize, size: usize) {
    let mut str_ptr = str_ptr as *const u8;
    for _ in 0..size {
        let char = MemoryMapping::<u8>::new(str_ptr as usize).read();
        uart::print_char(char as char);
        str_ptr = str_ptr.add(1);
    }
}

/// Returns true if user prog holds uart and blocks the process. Returns false otherwise.
unsafe fn get_char() -> Option<usize> {
    let user_prog = scheduler::cur();
    if !uart::is_open(user_prog) {
        return Some(0);
    }
    if let Some(char) = uart::get_char() {
        return Some(char as usize);
    }
    scheduler::cur().set_blocked(scheduler::Reason::Uart);
    sys_yield();
    return None;
}

unsafe fn exit() {
    let cur = scheduler::cur();
    uart::close(cur);
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
