use super::riscv;

static mut SETUP: bool = false;

pub fn setup() {
    unsafe {
        if SETUP {
            return;
        }
        SETUP = true;
    }
    let mstatus = riscv::read_mstatus();
    
}
