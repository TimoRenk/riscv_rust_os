//! Global kernel setup.

use crate::hardware::binary_struct::BinaryStruct;
use crate::{asm, hardware};
use riscv_utils::*;

/// Global kernel setup. It must only be called once.
pub unsafe fn setup() {
    // Set previous privilege mode to user so mret returns to user mode.
    let mstatus: usize;
    read_machine_reg!("mstatus" => mstatus);
    let mut mstatus = BinaryStruct::from(mstatus);
    mstatus.write_register_entry(MSTATUS_MPP_U.0);
    mstatus.write_register_entry(MSTATUS_MPP_U.1);

    // Enable machine-mode interrupts.
    mstatus.write_register_entry(MSTATUS_MIE);
    write_machine_reg!(mstatus.into_inner() => "mstatus");

    // Set the machine-mode trap handler.
    let trap_handler = asm::exception as usize;
    // Disable paging for now.
    let paging = 0usize;
    write_machine_reg!(
        trap_handler => "mtvec",
        paging => "satp"
    );
    // Init timer interrupt.
    hardware::clint::init();
    // Init hardware interrupt.
    hardware::plic::init();
    hardware::uart::init();
    // Configure physical memory protection.
    hardware::pmp::init();
    // Enable software interrupts (ecall) in M mode. Enable timer interrupts.
    let mie: usize;
    read_machine_reg!("mie" => mie);
    let mut mie = BinaryStruct::from(mie);
    mie.write_register_entry(MIE_MSIE);
    mie.write_register_entry(MIE_MTIE);
    mie.write_register_entry(MIE_MEIE);
    write_machine_reg!(mie.into_inner() => "mie");
}
