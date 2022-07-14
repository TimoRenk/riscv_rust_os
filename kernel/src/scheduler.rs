use crate::{
    hardware::{clint, pmp},
    user_prog,
};
use riscv_utils::*;

static mut CUR_PROG_IDX: usize = 0;
const NONE: Option<ProgData> = None;
static mut PROGS: [Option<ProgData>; 2] = [NONE; 2];

pub unsafe fn boot_prog(prog: Prog) {
    let prog_data = prog.get();
    prog_data.state = State::Rdy;
    switch(prog);
    riscv_utils::write_machine_reg!(prog_data.info.boot_mepc => "mepc");
    crate::println!("\n\n## Starting {:?} ##", prog_data.info.id);
    clint::set_time_cmp();
    core::arch::asm!("mret");
}
pub unsafe fn end_prog(prog: Prog) {
    prog.get();
    PROGS[prog.idx] = None;
}
pub unsafe fn init_prog(prog_info: user_prog::Info) -> Prog {
    let idx = get_free_idx();
    PROGS[idx] = Some(ProgData::new(prog_info));
    Prog {
        idx,
        id: prog_info.id,
    }
}
/// Returns the current user prog.
pub fn cur() -> Prog {
    unsafe {
        if let Some(cur) = &mut PROGS[CUR_PROG_IDX] {
            return Prog {
                idx: CUR_PROG_IDX,
                id: cur.info.id,
            };
        }
        panic!("Tried to access current user prog. But none was running");
    }
}
/// Returns the next rdy or starting user prog after round robin.
pub fn next() -> Option<Prog> {
    unsafe {
        let start = CUR_PROG_IDX + 1;
        for i in 0..PROGS.len() {
            let idx = (start + i) % PROGS.len();
            if let Some(next) = &mut PROGS[idx] {
                if next.state == State::Rdy || next.state == State::Starting {
                    return Some(Prog {
                        idx,
                        id: next.info.id,
                    });
                }
            }
        }
    }
    return None;
}
/// Switches the program.
pub fn switch(prog: Prog) {
    unsafe {
        let prog_data = prog.get();
        match prog_data.state {
            State::Rdy => {
                CUR_PROG_IDX = prog.idx;
                pmp::switch_prog_pmp(prog_data.info.pmp_idx);
            }
            State::Starting => {
                boot_prog(prog);
            }
            State::Blocked(_) => {
                panic!(
                    "Tried to switch to user prog: {:?}, with state: {:?}",
                    prog_data.info.id, prog_data.state
                )
            }
        }
    }
}
/// Safes the user prog.
pub fn save_cur_prog(mepc: usize, sp: usize) {
    unsafe {
        if mepc < 0x80100000usize {
            let mcause: usize;
            read_machine_reg!("mcause" => mcause);

            panic!("Interrupt in exception, mepc: {}, mcause: {}", mepc, mcause);
        }
        let prog = cur().get();
        prog.mepc = mepc;
        prog.sp = sp;
    }
}
/// Returns the stack pointer to restore it.
pub fn restore_cur_prog() -> usize {
    unsafe {
        let prog = cur().get();
        if prog.state == State::Rdy {
            write_machine_reg!(prog.mepc => "mepc");
            return prog.sp;
        }
        panic!(
            "Tried to restore user prog: {:?}, with state: {:?}",
            prog.info.id, prog.state
        );
    }
}
fn get_free_idx() -> usize {
    unsafe {
        for idx in 0..PROGS.len() {
            if PROGS[idx].is_none() {
                return idx;
            }
        }
    }
    panic!("No free index for user prog available");
}
#[derive(PartialEq, Clone, Copy)]
pub struct Prog {
    idx: usize,
    id: user_prog::Id,
}
impl Prog {
    unsafe fn get(&self) -> &'static mut ProgData {
        if let Some(cur) = &mut PROGS[self.idx] {
            if cur.info.id == self.id {
                return cur;
            }
            panic!(
                "Tried to access a user prog: {:?}, at: {}, but a different user prog was found: {:?}",
                self.id, self.idx, cur.info.id
            );
        }
        panic!(
            "Tried to access a not existing user prog: {:?}, at: {}",
            self.id, self.idx
        );
    }
    pub fn is_rdy(&self) -> bool {
        unsafe { self.get().state == State::Rdy }
    }
    pub fn set_rdy(&mut self) {
        unsafe {
            self.get().state = State::Rdy;
        }
    }
    /// If blocked, returns the reason. Otherwise None.
    pub fn is_blocked(&self) -> Option<Reason> {
        unsafe {
            if let State::Blocked(reason) = self.get().state {
                return Some(reason);
            }
        }
        return None;
    }
    pub fn set_blocked(&mut self, reason: Reason) {
        unsafe {
            self.get().state = State::Blocked(reason);
        }
    }
    pub fn increment_mepc(&mut self) {
        unsafe {
            self.get().mepc += 4;
        }
    }
    pub fn id(&self) -> user_prog::Id {
        unsafe { self.get().info.id }
    }
    pub fn prog_info(&self) -> user_prog::Info {
        unsafe { self.get().info }
    }
}
#[derive(PartialEq)]
struct ProgData {
    info: user_prog::Info,
    mepc: usize,
    sp: usize,
    state: State,
}
impl ProgData {
    fn new(prog_info: user_prog::Info) -> Self {
        ProgData {
            info: prog_info,
            sp: 0,
            mepc: 0,
            state: State::Starting,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum State {
    Rdy,
    Blocked(Reason),
    Starting,
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Reason {
    Uart,
}
