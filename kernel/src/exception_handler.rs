use core::arch::asm;

use crate::hardware::binary_struct::BinaryStruct;

use super::system_calls;
use riscv_utils::*;

#[no_mangle]
unsafe extern "C" fn exception_handler() {
    let number: u64;
    let param_0;
    let param_1;
    read_function_reg!(
        "a7" => number,
        "a1" => param_1,
        "a0" => param_0
    );
    let mcause: u64;
    read_machine_reg!("mcause" => mcause);
    // Interrupt
    if BinaryStruct::from(mcause).is_set(63) {
        return;
    }
    // Instruction access fault
    if mcause == 1 {
        let mepc: u64;
        let mtval: u64;
        read_machine_reg!("mepc" => mepc, "mtval" => mtval);
        system_calls::print_str("\n### Instruction access fault ###\n  mepc: ");
        system_calls::print_num(mepc);
        system_calls::print_str("\n  mtval: ");
        system_calls::print_num(mtval);
        system_calls::exit();
        return;
    }
    // Ecall from user-mode
    if mcause == 8 {
        system_calls::syscall(number, param_0, param_1);
    }
}
