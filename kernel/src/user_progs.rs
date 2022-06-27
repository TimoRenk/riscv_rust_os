use crate::hardware::pmp::switch_pmp;
use core::arch::asm;
use riscv_utils::write_machine_reg;

pub enum Progs {
    User1,
    User2,
}

pub unsafe fn switch_prog(prog: Progs) {
    let mepc;
    match prog {
        Progs::User1 => {
            mepc = 0x80100000u64;
        }
        Progs::User2 => {
            mepc = 0x80200000u64;
        }
    }
    switch_pmp(prog);
    write_machine_reg!(mepc => "mepc");
}
