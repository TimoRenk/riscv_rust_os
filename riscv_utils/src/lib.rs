//! RISC-V utilities. Shared between the kernel and the user programs.

#![no_std]
#![allow(unused)]
mod sys_call;
pub use sys_call::SysCall;

pub type RegisterEntry = (usize, bool);
///`mpp`: sets previous privilege mode to user-mode so modules run only in U-mode after the setup.
pub const MSTATUS_MPP_U: (RegisterEntry, RegisterEntry) = ((11, false), (12, false));
///`mie`: machine-mode interrupt enable
pub const MSTATUS_MIE: RegisterEntry = (3, true);

/// `meie`: external machine-mode interrupt enable
pub const MIE_MEIE: RegisterEntry = (11, true);
/// `mtie`: timer machine-mode interrupt enable
pub const MIE_MTIE: RegisterEntry = (7, true);
/// `msie`: software machine-mode interrupt enable
pub const MIE_MSIE: RegisterEntry = (3, true);

/// `seie`: external supervisor-mode interrupt enable
pub const SIE_SEIE: RegisterEntry = (9, true);
/// `stie`: timer supervisor-mode interrupt enable
pub const SIE_STIE: RegisterEntry = (5, true);
/// `ssie`: software supervisor-mode interrupt enable
pub const SIE_SSIE: RegisterEntry = (1, true);

/// `mcause` value for a timer interrupt.
pub const MCAUSE_INTERRUPT_TIMER: usize = 7;
/// `mcause` value for an extern interrupt, like the plic.
pub const MCAUSE_INTERRUPT_EXTERN: usize = 11;
/// `mcause` value for an instruction access fault exception.
pub const MCAUSE_EXCEPTION_IAF: usize = 1;
/// `mcause` value for an illegal instruction exception.
pub const MCAUSE_EXCEPTION_II: usize = 2;
/// `mcause` value for an load access fault exception.
pub const MCAUSE_EXCEPTION_LAF: usize = 5;
/// `mcause` value for an ecall exception.
pub const MCAUSE_EXCEPTION_ECALL: usize = 8;

/// A convenient macro to avoid writing assembly code for reading machine register.
///
/// ## Example
///
/// ```
/// let machine_status: usize;
/// let mie: usize;
/// read_machine_reg!("mstatus" => machine_status, "mie" => mie);
/// ```
///
/// The resulting code after the macro expansion:
///
/// ```
/// core::arch::asm!("csrr {}, mstatus", "csrr {}, mie", out(reg) machine_status, out(reg) mie);
/// ```
///
/// ## Common Machine Registers
///
/// - `mstatus`:   *machine status*
/// - `mepc`:      *machine exception program counter* holds the *return from exception* address.
/// - `satp`:      *supervisor address translation and protection* holds the *page table* address.
/// - `mie`:       *machine interrupt enable*
/// - `sie`:       *supervisor interrupt enable*
/// - `mtvec`:     *machine-mode interrupt vector*
/// - `pmpcfg0`
/// - `pmpaddr0`
#[macro_export]
macro_rules! read_machine_reg {
    ($($register:literal => $data:ident), +) => {
        core::arch::asm!(
            $(concat!("csrr {}, ", $register)), +,
            $(out(reg) $data), +
        )
    }
}

/// A convenient macro to avoid writing assembly code for writing machine register.
///
/// ## Example
///
/// ```
/// let trap_handler = 0usize;
/// let paging = 0usize;
/// write_machine_reg!(
///     trap_handler => "mtvec",
///     paging => "satp"
/// );
/// ```
///
/// The resulting code after the macro expansion:
///
/// ```
/// core::arch::asm!("csrw mtvec, {}", "csrw satp, {}", in(reg) trap_handler, in(reg) paging);
/// ```
///
/// ## Common Machine Registers
///
/// - `mstatus`:   *machine status*
/// - `mepc`:      *machine exception program counter* holds the *return from exception* address.
/// - `satp`:      *supervisor address translation and protection* holds the *page table* address.
/// - `mie`:       *machine interrupt enable*
/// - `sie`:       *supervisor interrupt enable*
/// - `mtvec`:     *machine-mode interrupt vector*
/// - `pmpcfg0`
/// - `pmpaddr0`
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

/// A convenient macro to avoid writing assembly code for reading function register.
///
/// The order seamed to matter at some point if multiple register where accessed.
/// If problems occur, try reading in descending register order.
///
/// ## Example
///
/// ```
/// let output;
/// riscv::read_function_reg!("a0" => output);
/// ```
///
/// The resulting code after the macro expansion:
///
/// ```
/// core::arch::asm!("mv {}, a0", out(reg) output);
/// ```
#[macro_export]
macro_rules! read_function_reg {
    ($($register:literal => $data:ident), +) => {
        core::arch::asm!(
            $(concat!("mv {}, ", $register)), +,
            $(out(reg) $data), +
        )
    }
}

/// A convenient macro to avoid writing assembly code for writing function register.
///
/// The order seamed to matter at some point if multiple register where accessed.
/// If problems occur, try writing in function parameter order.
///
/// ## Example
///
/// ```
/// let syscall = 3 as usize;
/// riscv::write_function_reg!(
///     syscall => "a7"
/// );
/// ```
///
/// The resulting code after the macro expansion:
///
/// ```
/// core::arch::asm!("mv a7, {}", in(reg) syscall);
/// ```
#[macro_export]
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
