use super::memory_mapping::MemoryMapping;

const TIMER_DURATION: u64 = 10000000; //10000000
const MTIMECMP_ADDR: usize = 0x0200_4000;
const MTIME_ADDR: usize = 0x0200_BFF8;

pub unsafe fn set_time_cmp() {
    let mut mtimecmp = MemoryMapping::new(MTIMECMP_ADDR);
    let mtime: u64 = MemoryMapping::new(MTIME_ADDR).read();
    mtimecmp.write(mtime + TIMER_DURATION);
}

pub unsafe fn init() {
    let mut mtimecmp = MemoryMapping::new(MTIMECMP_ADDR);
    mtimecmp.write(u64::MAX);
}
