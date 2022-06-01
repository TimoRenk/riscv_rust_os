use crate::kernel::asm;

use super::binary_struct::BinaryStruct;
use crate::riscv::*;
use core::arch::asm;

static mut SETUP: bool = false;

pub unsafe fn setup() {
    if SETUP {
        return;
    }
    SETUP = true;
    // set M Previous Privilege mode to User so mret returns to user mode.
    let mstatus: u64;
    read_machine_reg!("mstatus" => mstatus);
    let mut mstatus = BinaryStruct::from(mstatus);
    mstatus.write_register_entry(MSTATUS_MPP_U.0);
    mstatus.write_register_entry(MSTATUS_MPP_U.1);

    // enable machine-mode interrupts.
    mstatus.write_register_entry(MSTATUS_MIE);
    let mstatus = mstatus.get();
    write_machine_reg!(mstatus => "mstatus");

    // enable software interrupts (ecall) in M mode.
    let mie: u64;
    read_machine_reg!("mie" => mie);
    let mut mie = BinaryStruct::from(mie);
    mie.write_register_entry(MIE_MSIE);
    write_machine_reg!(mie.get() => "mie");

    // set the machine-mode trap handler.
    let trap_handler = asm::exception as u64;
    // disable paging for now.
    let paging = 0;
    // configure Physical Memory Protection to give user mode access to all of physical memory.
    let pmp_addr_0 = 0x3fffffffffffff;
    let pmpcfg0 = 0xf;
    // Machine exception ptr for mret, requires gcc -mcmodel=medany
    let program_ptr = crate::user::main as u64;
    write_machine_reg!(trap_handler => "mtvec",
        paging => "satp",
        pmp_addr_0 => "pmpaddr0",
        pmpcfg0 => "pmpcfg0",
        program_ptr => "mepc"
    );

    // switch to user mode (configured in mstatus) and jump to address in mepc CSR -> main().
    asm!("mret");
}
