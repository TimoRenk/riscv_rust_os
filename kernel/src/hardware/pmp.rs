use riscv_utils::write_machine_reg;

use super::binary_struct::Byte;
use crate::user_progs::Prog;

pub unsafe fn init() {
    let pmp_addr_0 = 0x80000000u64 >> 2; // devices
    let pmp_addr_1 = 0x80100000u64 >> 2; // kernel
    let pmp_addr_2 = 0x80200000u64 >> 2; // user1
    let pmp_addr_3 = 0x80300000u64 >> 2; // user2
    let pmp_addr_4 = 0x80400000u64 >> 2; // end
    let pmpcfg0 = 0;

    write_machine_reg!(
        pmp_addr_0 => "pmpaddr0",
        pmp_addr_1 => "pmpaddr1",
        pmp_addr_2 => "pmpaddr2",
        pmp_addr_3 => "pmpaddr3",
        pmp_addr_4 => "pmpaddr4",
        pmpcfg0 => "pmpcfg0"
    );
}

pub unsafe fn switch_pmp(prog: Prog) {
    let mut pmpcfg0 = Pmpcfg::new();
    let prog_index = prog as usize + 2; // device and kernel offset
    pmpcfg0.set_rwx(prog_index);
    write_machine_reg!(pmpcfg0.to_u64() => "pmpcfg0");
}

#[repr(C)]
struct Pmpcfg([Byte; 8]);
impl Pmpcfg {
    fn to_u64(&self) -> u64 {
        let mut arr = [0; 8];
        for i in 0..self.0.len() {
            arr[i] = self.0[i].get();
        }
        return u64::from_ne_bytes(arr);
    }
    fn set_rwx(&mut self, at: usize) {
        let reg = &mut self.0[at];
        reg.at(0, true); // R
        reg.at(1, true); // W
        reg.at(2, true); // X
        reg.at(3, true); // A - top of range
    }
    fn new() -> Self {
        let bytes = [Byte::from(0); 8];
        Pmpcfg(bytes)
    }
}
