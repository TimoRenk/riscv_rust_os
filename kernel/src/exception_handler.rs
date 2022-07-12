use crate::{
    hardware::{binary_struct::BinaryStruct, clint, memory_mapping::MemoryMapping},
    user_prog,
};

use super::system_calls;
use riscv_utils::*;

#[no_mangle]
unsafe extern "C" fn exception_handler(mepc: usize, mcause: usize, sp: usize) {
    user_prog::save_prog(mepc, sp);
    let mut mcause = BinaryStruct::from(mcause);
    let interrupt = mcause.is_set(63);
    if interrupt {
        mcause.at(63, false);
        handle_interrupt(mcause.get());
    } else {
        handle_exception(mcause.get(), mepc, sp);
    }
    let sp = user_prog::restore_prog();
    write_function_reg!(sp=> "a7");
}

unsafe fn handle_interrupt(mcause: usize) {
    match mcause {
        7 => {
            // Timer interrupt
            let next = user_prog::get();
            user_prog::switch_or_start(next);
            clint::set_time_cmp();
        }
        11 => {
            // Extern interrupt
        }
        _ => {
            panic!("Unsupported Interrupt with code: {}", mcause);
        }
    }
}

unsafe fn handle_exception(mcause: usize, mepc: usize, sp: usize) {
    match mcause {
        1 => {
            // Instruction access fault
            let mtval: usize;
            read_machine_reg!("mtval" => mtval);
            panic!(
                "Instruction access fault in user prog: {:?}, mepc: 0x{:x}, mtval: 0x{:x}",
                user_prog::get(),
                mepc,
                mtval
            );
        }
        5 => {
            // Load access fault
            let mtval: usize;
            read_machine_reg!("mtval" => mtval);
            panic!(
                "Load access fault in user prog: {:?}, mepc: 0x{:x}, mtval: 0x{:x}",
                user_prog::get(),
                mepc,
                mtval
            );
        }
        8 => {
            // Ecall from user-mode
            let stack: Stack = MemoryMapping::new(sp as usize).read();
            let number = stack[16];
            let param_0 = stack[9];
            let param_1 = stack[10];
            system_calls::syscall(number, param_0, param_1);
        }
        _ => {
            // Unsupported exception
            panic!("Unsupported exception with code: {}", mcause);
        }
    }
}
type Stack = [usize; 32];
