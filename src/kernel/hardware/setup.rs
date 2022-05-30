use crate::kernel::asm;

use super::binary_struct::BinaryStruct;
use super::riscv::{Register as Reg, *};
use core::arch::asm;

static mut SETUP: bool = false;

pub fn setup() {
    unsafe {
        if SETUP {
            return;
        }
        SETUP = true;
    }
    // set M Previous Privilege mode to User so mret returns to user mode.
    let mut mstatus = BinaryStruct::from(read_register(Reg::MStatus));
    mstatus.write_register_entry(MSTATUS_MPP_U.0);
    mstatus.write_register_entry(MSTATUS_MPP_U.1);
    // enable machine-mode interrupts.
    mstatus.write_register_entry(MSTATUS_MIE);
    write_register(Reg::MStatus, mstatus.get());
    // enable software interrupts (ecall) in M mode.
    let mut mie = BinaryStruct::from(read_register(Reg::MIE));
    mie.write_register_entry(MIE_MSIE);
    write_register(Reg::MIE, mie.get());
    // set the machine-mode trap handler to jump to function "NAME" when a trap occurs.
    write_register(Reg::MTVec, asm::exception as u64);
    // disable paging for now.
    write_register(Reg::SATP, 0);
    // configure Physical Memory Protection to give user mode access to all of physical memory.
    write_register(Reg::PmpAddr0, 0x3fffffffffffff);
    write_register(Reg::PmpCfg0, 0xf);
    // set M Exception Program Counter to main, for mret, requires gcc -mcmodel=medany
    write_register(Reg::MEPC, crate::user::main as u64);
    // switch to user mode (configured in mstatus) and jump to address in mepc CSR -> main().
    unsafe {
        asm!("mret");
    }
}
