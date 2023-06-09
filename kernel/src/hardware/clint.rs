//!  clint -- Core Local Interrupt

use super::memory_mapping::MemoryMapping;

const TIMER_DURATION: u64 = 10000000;

///     `mtimecmp_addr`: Address of the Compare Value for the Core Local Interrupt (clint), triggers timer interrupt **!! In QEMU at 0x0200 on real hardware at 0x2000**
pub const MTIMECMP_ADDR: usize = 0x0200_4000;

///     `mtime`: 64bit register of the timer incremented every clock-cycle e.g. 10.000.000 times on QEMU with 10Mhz
pub const MTIME_ADDR: usize = 0x0200_BFF8;

pub fn set_time_cmp() {
    unsafe {
        let mtimecmp = MemoryMapping::new(MTIMECMP_ADDR);
        let mtime: u64 = MemoryMapping::new(MTIME_ADDR).read();
        mtimecmp.write(mtime + TIMER_DURATION);
    }
}

pub fn init() {
    unsafe {
        let mtimecmp = MemoryMapping::new(MTIMECMP_ADDR);
        mtimecmp.write(u64::MAX);
    }
}
