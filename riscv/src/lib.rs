#![no_std]
#![allow(dead_code)]
mod sys_call;
pub use sys_call::SysCall;

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

pub type RegisterEntry = (usize, bool);

// MStatus, // Machine Status
// MEPC,    // 'machine exception program counter' holds the 'return from exception' address.
// SATP,    // 'supervisor address translation and protection' holds the 'page table' address.
// MIE,     // 'machine interrupt enable'
// SIE,     // 'supervisor interrupt enable'
// MTVec,   // 'machine-mode interrupt vector'
// PmpCfg0,
// PmpAddr0,
#[macro_export]
macro_rules! read_machine_reg {
    ($($register:literal => $data:ident), +) => {
        core::arch::asm!(
            $(concat!("csrr ", "{", stringify!($data), "}, ", $register)), +,
            $($data = out(reg) $data), +
        )
    }
}
#[macro_export]
macro_rules! write_machine_reg {
    ($($data:ident => $register:literal), +) => {
        $(let $data: usize = $data;) +
        core::arch::asm!(
            $(concat!("csrw ", $register, ", {}")), +,
            $(in(reg) $data), +
        )
    };
    ($data:expr => $register:literal) => {
        let data: usize = $data;
        core::arch::asm!(concat!("csrw ", $register, ", {}"), in(reg) data)
    };
}
///!!!ALWAYS read in descending register order!!!
#[macro_export]
macro_rules! read_function_reg {
    ($($register:literal => $data:ident), +) => {
        core::arch::asm!(
            $(concat!("mv ", "{}, ", $register)), +,
            $(out(reg) $data), +
        )
    }
}
#[macro_export]
///!!!ALWAYS write in function parameter order!!!
macro_rules! write_function_reg {
    ($($data:ident => $register:literal), +) => {
        core::arch::asm!(
            $(concat!("mv ", $register, ", {}")), +,
            $(in(reg) $data), +
        )
    };
    ($data:expr => $register:literal) => {
        let data: u64 = $data;
        core::arch::asm!(concat!("mv ", $register, ", {}"), in(reg) data)
    };
}
