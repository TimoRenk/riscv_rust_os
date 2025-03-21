//! The kernel side of the system calls.

use riscv_utils::*;

use super::hardware::{memory_mapping::MemoryMapping, uart};
use crate::scheduler;

fn sys_call_from(number: usize) -> SysCall {
    SysCall::try_from(number as isize)
        .unwrap_or_else(|_| panic!("Illegal syscall number: {}", number))
}

pub fn sys_call(number: usize, param_0: usize, param_1: usize) -> Option<usize> {
    match sys_call_from(number) {
        SysCall::PrintString => {
            unsafe { print_string(param_0, param_1) };
            scheduler::cur().increment_mepc();
            None
        }
        SysCall::PrintChar => {
            uart::print_char(param_0 as u8 as char);
            scheduler::cur().increment_mepc();
            None
        }
        SysCall::GetChar => {
            scheduler::cur().increment_mepc();
            get_char()
        }
        SysCall::PrintNum => {
            uart::print_num(param_0);
            scheduler::cur().increment_mepc();
            None
        }
        SysCall::Exit => {
            exit();
            None
        }
        SysCall::Yield => {
            scheduler::cur().increment_mepc();
            sys_yield();
            None
        }
        SysCall::UartOpen => {
            let open = uart::open(scheduler::cur());
            scheduler::cur().increment_mepc();
            Some(open as usize)
        }
        SysCall::UartClose => {
            let close = uart::close(scheduler::cur());
            scheduler::cur().increment_mepc();
            Some(close as usize)
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

/// Returns [Some] if user prog holds uart and blocks the process.
/// Returns [None] otherwise.
fn get_char() -> Option<usize> {
    let user_prog = scheduler::cur();
    if !uart::is_open(user_prog) {
        return Some(0);
    }
    if let Some(char) = uart::get_char() {
        return Some(char as usize);
    }
    scheduler::cur().set_blocked(scheduler::Reason::Uart);
    sys_yield();
    None
}

fn exit() {
    let cur = scheduler::cur();
    uart::close(cur);
    let prog_info = cur.prog_info();
    scheduler::end_prog(scheduler::cur());
    scheduler::init_prog(prog_info);
    sys_yield();
}

fn sys_yield() {
    let next =
        scheduler::next().expect("No next user prog for system yield. Idle task not implemented");
    scheduler::switch(next);
}
