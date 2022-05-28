use core::arch::asm;

/// Reads the 'machine status'.
pub fn read_mstatus() -> u64 {
    let mstatus: u64;
    unsafe { asm!("csrr {}, mstatus", out(reg) mstatus) }
    mstatus
}

/// Writes the 'machine status'.
pub fn write_mstatus(mstatus: u64) {
    unsafe { asm!("csrw mstatus, {}", in(reg) mstatus) }
}

/// Reads the 'machine exception program counter' holds the 'return from exception' address.
pub fn read_mepc() -> u64 {
    let mepc: u64;
    unsafe { asm!("csrr {}, mepc", out(reg) mepc) }
    mepc
}

/// Writes the 'machine exception program counter' holds the 'return from exception' address.
pub fn write_mepc(mepc: u64) {
    unsafe { asm!("csrw mepc, {}", in(reg) mepc) }
}

/// Writes the 'supervisor address translation and protection' holds the 'page table' address.
pub fn write_satp(satp: u64) {
    unsafe { asm!("csrw satp, {}", in(reg) satp) }
}

/// Reads the 'machine interrupt enable'.
pub fn read_mie() -> u64 {
    let mie: u64;
    unsafe { asm!("csrr {}, mie", out(reg) mie) }
    mie
}

/// Writes the 'machine interrupt enable'.
pub fn write_mie(mie: u64) {
    unsafe { asm!("csrw mie, {}", in(reg) mie) }
}

/// Reads the 'supervisor interrupt enable'.
pub fn read_sie() -> u64 {
    let sie: u64;
    unsafe { asm!("csrr {}, sie", out(reg) sie) }
    sie
}

/// Writes the 'supervisor interrupt enable'.
pub fn write_sie(sie: u64) {
    unsafe { asm!("csrw sie, {}", in(reg) sie) }
}

/// Writes the 'machine-mode interrupt vector'.
pub fn write_mtvec(mtvec: u64) {
    unsafe { asm!("csrw mtvec, {}", in(reg) mtvec) }
}
