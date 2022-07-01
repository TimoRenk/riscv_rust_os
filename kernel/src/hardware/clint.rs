use super::memory_mapping::MemoryMapping;

const TIMER_DURATION: u64 = 5000; //00000
const MTIMECMP_ADDR: usize = 0x0200_4000;
const MTIME_ADDR: usize = 0x0200_BFF8;

pub fn set_time_cmp() {
    let mut mtimecmp: MemoryMapping<u64> = MemoryMapping::new(MTIMECMP_ADDR);
    let mtime: u64 = *MemoryMapping::new(MTIME_ADDR).get();
    *mtimecmp.get() = mtime + TIMER_DURATION;
}

pub fn init_timer() {
    let mut mtimecmp: MemoryMapping<u64> = MemoryMapping::new(MTIMECMP_ADDR);
    *mtimecmp.get() = u64::MAX;
}
