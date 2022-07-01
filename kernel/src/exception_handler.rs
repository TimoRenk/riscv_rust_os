use crate::{
    hardware::{binary_struct::BinaryStruct, clint, memory_mapping::MemoryMapping},
    user_prog,
};

use super::system_calls;
use riscv_utils::*;

#[no_mangle]
unsafe extern "C" fn exception_handler() {
    let user_prog::ProgReg { mepc, sp } = user_prog::save_prog();
    let mcause: u64;
    read_machine_reg!("mcause" => mcause);
    let mut mcause = BinaryStruct::from(mcause);
    let interrupt = mcause.is_set(63);
    if interrupt {
        mcause.at(63, false);
        match mcause.get() {
            7 => {
                let next = user_prog::get();
                user_prog::switch_or_start(next);
                clint::set_time_cmp();
            }
            _ => {
                panic!("Unsupported Interrupt with code: {}", mcause.get());
            }
        }
    } else {
        match mcause.get() {
            1 => {
                // Instruction access fault
                let mtval: u64;
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
                let mtval: u64;
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
                let stack: Stack = *MemoryMapping::new(sp as usize).get();
                let number = stack[16];
                let param_0 = stack[9];
                let param_1 = stack[10];
                system_calls::syscall(number, param_0, param_1);
            }
            _ => {
                // Unsupported exception
                panic!("Unsupported exception with code: {}", mcause.get());
            }
        }
    }
    user_prog::restore_prog();
}

type Stack = [u64; 32];
