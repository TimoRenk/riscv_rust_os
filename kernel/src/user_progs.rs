mod user_1;
mod user_2;

use crate::hardware::memory_mapping::MemoryMapping;

const USER_PROG_ADDR: usize = 0x8010_0000;

pub enum Progs {
    User1,
    User2,
}

pub fn switch_prog(prog: Progs) {
    match prog {
        Progs::User1 => copy_prog(user_1::USER_PROG_1),
        Progs::User2 => copy_prog(user_2::USER_PROG_2),
    }
}

fn copy_prog(prog: [u8; 4096]) {
    let mut user_prog: MemoryMapping<[u8; 4096]> = MemoryMapping::new(USER_PROG_ADDR);
    *user_prog.get() = prog;
}
