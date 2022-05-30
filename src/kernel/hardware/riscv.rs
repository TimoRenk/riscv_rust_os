#![allow(dead_code)]
#![allow(unused_variables)]
use core::arch::asm;

#[warn(dead_code)]
use super::binary_struct::RegisterEntry;
type RegEnt = RegisterEntry;
/// mstatus: machine status
///     mpp: the previous mode
///         u: User
///     mie: machine-mode interrupt enable
pub const MSTATUS_MPP_U: (RegEnt, RegEnt) = ((11, false), (12, false));
pub const MSTATUS_MIE: RegEnt = (3, true);
/// mie: machine-mode interrupt
///     meie: external
///     mtie: timer
///     msie: software
pub const MIE_MEIE: RegEnt = (11, true);
pub const MIE_MTIE: RegEnt = (7, true);
pub const MIE_MSIE: RegEnt = (3, true);
/// sie: supervisor interrupt enable
///     seie: external
///     stie: times
///     ssie: software
pub const SIE_SEIE: RegEnt = (9, true);
pub const SIE_STIE: RegEnt = (5, true);
pub const SIE_SSIE: RegEnt = (1, true);

pub enum Register {
    MStatus, // Machine Status
    MEPC,    // 'machine exception program counter' holds the 'return from exception' address.
    SATP,    // 'supervisor address translation and protection' holds the 'page table' address.
    MIE,     // 'machine interrupt enable'
    SIE,     // 'supervisor interrupt enable'
    MTVec,   // 'machine-mode interrupt vector'
    PmpCfg0,
    PmpAddr0,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
}
/// Read register
pub fn read_register(register: Register) -> u64 {
    let output: u64;
    unsafe {
        match register {
            Register::MStatus => asm!("csrr {}, mstatus", out(reg) output),
            Register::MEPC => asm!("csrr {}, mepc", out(reg) output),
            Register::SATP => asm!("csrr {}, satp", out(reg) output),
            Register::MIE => asm!("csrr {}, mie", out(reg) output),
            Register::SIE => asm!("csrr {}, sie", out(reg) output),
            Register::MTVec => asm!("csrr {}, mtvec", out(reg) output),
            Register::PmpCfg0 => asm!("csrr {}, pmpcfg0", out(reg) output),
            Register::PmpAddr0 => asm!("csrr {}, pmpaddr0", out(reg) output),
            Register::A0 => asm!("add {}, a0, zero", out(reg) output),
            Register::A1 => asm!("add {}, a1, zero", out(reg) output),
            Register::A2 => asm!("add {}, a2, zero", out(reg) output),
            Register::A3 => asm!("add {}, a3, zero", out(reg) output),
            Register::A4 => asm!("add {}, a4, zero", out(reg) output),
            Register::A5 => asm!("add {}, a5, zero", out(reg) output),
            Register::A6 => asm!("add {}, a6, zero", out(reg) output),
            Register::A7 => asm!("add {}, a7, zero", out(reg) output),
        }
    }
    output
}
/// Write register
pub fn write_register(register: Register, input: u64) {
    unsafe {
        match register {
            Register::MStatus => asm!("csrw mstatus, {}", in(reg) input),
            Register::MEPC => asm!("csrw mepc, {}", in(reg) input),
            Register::SATP => asm!("csrw satp, {}", in(reg) input),
            Register::MIE => asm!("csrw mie, {}", in(reg) input),
            Register::SIE => asm!("csrw sie, {}", in(reg) input),
            Register::MTVec => asm!("csrw mtvec, {}", in(reg) input),
            Register::PmpCfg0 => asm!("csrw pmpcfg0, {}", in(reg) input),
            Register::PmpAddr0 => asm!("csrw pmpaddr0, {}", in(reg) input),
            Register::A0 => asm!("add a0, {}, zero", in(reg) input),
            Register::A1 => asm!("add a1, {}, zero", in(reg) input),
            Register::A2 => asm!("add a2, {}, zero", in(reg) input),
            Register::A3 => asm!("add a3, {}, zero", in(reg) input),
            Register::A4 => asm!("add a4, {}, zero", in(reg) input),
            Register::A5 => asm!("add a5, {}, zero", in(reg) input),
            Register::A6 => asm!("add a6, {}, zero", in(reg) input),
            Register::A7 => asm!("add a7, {}, zero", in(reg) input),
        }
    }
}

/// Writes the 'machine-mode interrupt vector'.
pub fn write_mtvec(mtvec: u64) {
    unsafe { asm!("csrw mtvec, {}", in(reg) mtvec) }
}
