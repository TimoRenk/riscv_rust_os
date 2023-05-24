use super::{binary_struct::BinaryStruct, memory_mapping::MemoryMapping};

//  plic - Platform-Level Interrupt Controller

/// Base-address for QEMU
///
/// [More Info](https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc#memory-map)
/// PLIC_MEMORY_MAP_BASE: usize = 0x0c00_0000;

/// Base address for the interrupt priorities, starts at `PLIC_MEMORY_MAP_BASE + 0x0000_0000`, consisting of 32-bit registers.
/// Priorities are unsigned u32, 0 means "never interrupt", max priority is platform specific. Note that *0x0000_0000* does not have an interrupt source since interrupt 0 does not exist
///
/// [More Info](https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc#interrupt-priorities)
pub const PRIORITY_BASE_ADDR: usize = 0x0c00_0000;

/// Base address for enabling interrupt sources, starts at `PLIC_MEMORY_MAP_BASE + 0x0000_2000`. 1-bit for enable of interrupt source with ID = bit position. Continuos block for 0-1023 sources for
/// 15872 contexts
///
/// [More Info](https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc#interrupt-enables)
pub const ENABLE_ADDR: usize = 0x0c00_2000;

/// Base address for setting of a interrupt priority threshold, starts at `PLIC_MEMORY_MAP_BASE + 0x0020_0000`, incremented by 0x1000 for each context.
/// PLIC ignorers all interrupts with priority less than or equal to the given threshold, set individually for all 15872 contexts.
///
/// [More Info](https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc#priority-thresholds)
pub const THRESHOLD_ADDR: usize = 0x0c20_0000;

/// Base address for the interrupt claim and completion registers, starts at `PLIC_MEMORY_MAP_BASE + 0x0020_0004`, incremented by 0x1000 for each context. If interrupt is handled by service after
/// receiving an interrupt notification, the Interrupt must be claimed from the PLIC. PLIC returns Interrupt ID to Service, if no interrupt is pending returns 0.
///
/// [More Info](https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc#interrupt-claim-process)
pub const CLAIM_COMP_ADDR: usize = 0x0c20_0004;

/// Interrupt request.
pub enum Irq {
    Uart = 10,
}

pub unsafe fn init() {
    let uart_priority_addr = get_priority_addr(Irq::Uart);
    MemoryMapping::new(uart_priority_addr).write(5);
    // Enable in context.
    let mut enable_c0 = [BinaryStruct::from(0u32); 32];
    let (uart_idx, uart_bit) = group_idx_and_bit_pos(Irq::Uart);
    enable_c0[uart_idx].at(uart_bit, true);
    let enable_addr = get_enable_addr(uart_idx);
    MemoryMapping::new(enable_addr).write(enable_c0[uart_idx].get());
    // Set thresholds for context.
    MemoryMapping::new(THRESHOLD_ADDR).write(0u32);
}

pub unsafe fn read_claim() -> Irq {
    let claim: u32 = MemoryMapping::new(CLAIM_COMP_ADDR).read();
    let claim = claim as usize;
    crate::enum_matching!(claim: Irq::Uart);
    panic!("Unknown plic interrupt request: {}", claim);
}

pub unsafe fn write_complete(irq: Irq) {
    MemoryMapping::new(CLAIM_COMP_ADDR).write(irq as u32);
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
