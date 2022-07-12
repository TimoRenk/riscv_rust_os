use crate::hardware::{clint, pmp::switch_pmp};
use riscv_utils::*;

#[derive(Clone, Copy, Debug)]
pub enum Prog {
    User1,
    User2,
}

static mut USER_PROG: Prog = Prog::User1;
static mut PROG_REGS: [Option<ProgReg>; 2] = [Option::None; 2];

pub unsafe fn start_prog(prog: Prog) {
    let mepc;
    match prog {
        Prog::User1 => {
            switch(prog);
            mepc = 0x80100000;
        }
        Prog::User2 => {
            switch(prog);
            mepc = 0x80200000;
        }
    }
    riscv_utils::write_machine_reg!(mepc => "mepc");
    crate::println!("\n\n## Starting {:?} ##", get());
    clint::set_time_cmp();
    core::arch::asm!("mret");
}
/// Switches the program or starts a new if it isn't running.
pub fn switch_or_start(prog: Prog) {
    if !is_started(prog) {
        unsafe {
            start_prog(prog);
        }
    }
    switch(prog);
}
/// Switches the program.
fn switch(prog: Prog) {
    unsafe {
        USER_PROG = prog;
        switch_pmp(prog);
    }
}
/// Safes the user prog.
pub fn save_prog(mepc: usize, sp: usize) -> ProgReg {
    if mepc < 0x80100000usize {
        let mcause: usize;
        unsafe {
            read_machine_reg!("mcause" => mcause);
        }
        panic!("Interrupt in exception, mepc: {}, mcause: {}", mepc, mcause);
    }
    let prog_reg = ProgReg { sp, mepc };
    write_prog_reg(prog_reg);
    return prog_reg;
}
/// Returns the stack pointer to restore it.
pub fn restore_prog() -> usize {
    if let Some(ProgReg { sp, mepc }) = read_prog_reg() {
        unsafe {
            write_machine_reg!(mepc => "mepc");
        }
        return sp;
    } else {
        panic!("Program register from user prog: {:?} not found", get());
    }
}
/// Returns the next user prog after round robin.
pub fn next() -> Prog {
    unsafe {
        match USER_PROG {
            Prog::User1 => Prog::User2,
            Prog::User2 => Prog::User1,
        }
    }
}

/// Returns the current user prog.
pub fn get() -> Prog {
    unsafe { USER_PROG }
}

fn read_prog_reg() -> Option<ProgReg> {
    unsafe { PROG_REGS[USER_PROG as usize] }
}

pub fn is_started(prog: Prog) -> bool {
    unsafe { PROG_REGS[prog as usize].is_some() }
}

pub fn increment_mepc() {
    if let Some(mut prog_reg) = read_prog_reg() {
        prog_reg.mepc = prog_reg.mepc + 4;
        write_prog_reg(prog_reg);
    } else {
        panic!("Program register from user prog: {:?} not found", get());
    }
}

fn write_prog_reg(reg: ProgReg) {
    unsafe {
        PROG_REGS[USER_PROG as usize] = Some(reg);
    }
}

#[derive(Clone, Copy)]
pub struct ProgReg {
    pub sp: usize,
    pub mepc: usize,
}
