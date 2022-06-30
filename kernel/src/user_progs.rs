use riscv_utils::*;

use crate::hardware::pmp::switch_pmp;

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
            switch_prog(prog);
            mepc = 0x80100000u64;
        }
        Prog::User2 => {
            switch_prog(prog);
            mepc = 0x80200000u64;
        }
    }
    riscv_utils::write_machine_reg!(mepc => "mepc");
    core::arch::asm!("mret");
}

pub fn switch_prog(prog: Prog) {
    unsafe {
        USER_PROG = prog;
        switch_pmp(prog);
    }
}

pub fn save_prog() -> ProgReg {
    let mepc: u64;
    let sp: u64;
    unsafe {
        read_function_reg!("s1" => sp);
        read_machine_reg!("mepc" => mepc);
    }

    let prog_reg = ProgReg { sp, mepc };
    write_prog_reg(prog_reg);
    return prog_reg;
}

pub fn restore_prog() {
    let ProgReg { sp, mepc } = *read_prog_reg();
    unsafe {
        write_machine_reg!(mepc => "mepc");
        write_function_reg!(sp => "s1");
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

fn read_prog_reg() -> &'static mut ProgReg {
    unsafe {
        let reg = &mut PROG_REGS[USER_PROG as usize];
        if let Some(reg) = reg {
            return reg;
        } else {
            panic!("Program register from user prog: {:?} not found", USER_PROG)
        }
    }
}

pub fn increment_mepc() {
    let ProgReg { mepc, sp: _ } = read_prog_reg();
    *mepc = *mepc + 4;
}

fn write_prog_reg(reg: ProgReg) {
    unsafe {
        match USER_PROG {
            Prog::User1 => PROG_REGS[0] = Some(reg),
            Prog::User2 => PROG_REGS[1] = Some(reg),
        }
    }
}

#[derive(Clone, Copy)]
pub struct ProgReg {
    pub sp: u64,
    pub mepc: u64,
}
