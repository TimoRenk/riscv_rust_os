//!  plic -- Platform-Level Interrupt Controller

use super::{binary_struct::BinaryStruct, memory_mapping::MemoryMapping};
use enum_matching::EnumTryFrom;

/// Base-address for QEMU
///
/// [More Info](https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc#memory-map)
const _PLIC_MEMORY_MAP_BASE: usize = 0x0c00_0000;

/// Base address for the interrupt priorities.
/// Starts at `_PLIC_MEMORY_MAP_BASE + 0x0000_0000` consisting of 32-bit registers.
/// Priorities are unsigned u32.
/// 0 means "never interrupt".
/// Max priority is platform specific.
/// Note that *0x0000_0000* does not have an interrupt source since interrupt 0 does not exist.
///
/// [More Info](https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc#interrupt-priorities)
pub const PRIORITY_BASE_ADDR: usize = 0x0c00_0000;

/// Base address for enabling interrupt sources.
/// Starts at `_PLIC_MEMORY_MAP_BASE + 0x0000_2000`.
/// 1-bit for enabling the interrupt source with ID = bit position.
/// Continuous block (0-1023) for 15872 contexts.
///
/// [More Info](https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc#interrupt-enables)
pub const ENABLE_ADDR: usize = 0x0c00_2000;

/// Base address for setting an interrupt priority threshold.
/// Starts at `_PLIC_MEMORY_MAP_BASE + 0x0020_0000`.
/// Incremented by 0x1000 for each context.
/// PLIC ignorers all interrupts with priority less than or equal to the given threshold.
/// Set individually for all 15872 contexts.
///
/// [More Info](https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc#priority-thresholds)
pub const THRESHOLD_ADDR: usize = 0x0c20_0000;

/// Base address for the interrupt claim and completion registers.
/// Starts at `_PLIC_MEMORY_MAP_BASE + 0x0020_0004`.
/// Incremented by 0x1000 for each context.
/// If an interrupt is handled by a service after receiving an interrupt notification the interrupt has to be claimed from the PLIC.
/// PLIC returns the interrupt ID to the service.
/// Returns 0 if no interrupt is pending.
///
/// [More Info](https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc#interrupt-claim-process)
pub const CLAIM_COMP_ADDR: usize = 0x0c20_0004;

/// Interrupt request.
#[derive(EnumTryFrom)]
pub enum Irq {
    Uart = 10,
}

pub fn init() {
    unsafe {
        let uart_priority_addr = get_priority_addr(Irq::Uart);
        MemoryMapping::new(uart_priority_addr).write(5);
        // Enable in context.
        let mut enable_c0 = [BinaryStruct::from(0u32); 32];
        let (uart_idx, uart_bit) = group_idx_and_bit_pos(Irq::Uart);
        enable_c0[uart_idx].at(uart_bit, true);
        let enable_addr = get_enable_addr(uart_idx);
        MemoryMapping::new(enable_addr).write(enable_c0[uart_idx].into_inner());
        // Set thresholds for context.
        MemoryMapping::new(THRESHOLD_ADDR).write(0u32);
    }
}

pub fn read_claim() -> Irq {
    unsafe {
        let claim: u32 = MemoryMapping::new(CLAIM_COMP_ADDR).read();
        let claim = claim as usize;
        Irq::try_from(claim as isize)
            .unwrap_or_else(|_| panic!("Unknown plic interrupt request: {}", claim))
    }
}

pub fn write_complete(irq: Irq) {
    unsafe {
        MemoryMapping::new(CLAIM_COMP_ADDR).write(irq as u32);
    }
}

fn get_priority_addr(irq: Irq) -> usize {
    PRIORITY_BASE_ADDR + 4 * irq as usize
}

/// Returns the (group index, bit position) of an irq if every bit is used as an id for an irq.
fn group_idx_and_bit_pos(irq: Irq) -> (usize, usize) {
    let irq = irq as usize;
    (irq / 32, (irq % 32))
}

fn get_enable_addr(idx: usize) -> usize {
    ENABLE_ADDR + 4 * idx
}
