//! The scheduler. Responsible for managing user programs.

use crate::{
    hardware::sync::Protected,
    hardware::{clint, pmp},
    user_prog,
};
use riscv_utils::*;

static PROG_LIST: Protected<ProgList> = Protected::new(ProgList::new());

pub fn boot_prog(prog: Prog) {
    PROG_LIST.lock().boot_prog(prog);
}
pub fn end_prog(prog: Prog) {
    let mut prog_list = PROG_LIST.lock();
    prog_list.get(prog); // Check if the prog has the correct index.
    prog_list.progs[prog.idx] = None;
}
pub fn init_prog(prog_info: user_prog::Info) -> Prog {
    let mut prog_list = PROG_LIST.lock();
    let idx = prog_list.get_free_idx();
    prog_list.progs[idx] = Some(ProgData::new(prog_info));
    Prog {
        idx,
        id: prog_info.id,
    }
}
/// Returns the current user prog.
pub fn cur() -> Prog {
    let prog_list = PROG_LIST.lock();
    if let Some(cur) = &prog_list.progs[prog_list.cur_prog_idx] {
        return Prog {
            idx: prog_list.cur_prog_idx,
            id: cur.info.id,
        };
    }
    panic!("Tried to access current user prog. But none was running");
}
/// Returns the next rdy or starting user prog after round robin.
pub fn next() -> Option<Prog> {
    let prog_list = PROG_LIST.lock();
    let start = prog_list.cur_prog_idx + 1;
    let prog_list_len = prog_list.progs.len();
    for i in 0..prog_list_len {
        let idx = (start + i) % prog_list_len;
        if let Some(next) = &prog_list.progs[idx] {
            if next.state == State::Rdy || next.state == State::Starting {
                return Some(Prog {
                    idx,
                    id: next.info.id,
                });
            }
        }
    }
    None
}
/// Switches the current program.
pub fn switch(prog: Prog) {
    PROG_LIST.lock().switch(prog);
}
/// Safes the user prog.
pub fn save_cur_prog(mepc: usize, sp: usize) {
    unsafe {
        if mepc < 0x80100000usize {
            let mcause: usize;
            read_machine_reg!("mcause" => mcause);

            panic!("Interrupt in exception, mepc: {}, mcause: {}", mepc, mcause);
        }
        let mut prog_list = PROG_LIST.lock();
        let prog = prog_list.cur_prog_data();
        prog.mepc = mepc;
        prog.sp = sp;
    }
}
/// Returns the stack pointer for restoring.
pub fn restore_cur_prog() -> usize {
    unsafe {
        let mut prog_list = PROG_LIST.lock();
        let prog = prog_list.cur_prog_data();
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
struct ProgList {
    cur_prog_idx: usize,
    progs: [Option<ProgData>; 2],
}
impl ProgList {
    const fn new() -> Self {
        ProgList {
            cur_prog_idx: 0,
            progs: [const { None }; 2],
        }
    }
    /// Switches the current program.
    fn switch(&mut self, prog: Prog) {
        let prog_data = self.get(prog);
        match prog_data.state {
            State::Rdy => {
                pmp::switch_prog_pmp(prog_data.info.pmp_idx);
                self.cur_prog_idx = prog.idx;
            }
            State::Starting => {
                self.boot_prog(prog);
            }
            State::Blocked(_) => {
                panic!(
                    "Tried to switch to user prog: {:?}, with state: {:?}",
                    prog_data.info.id, prog_data.state
                )
            }
        }
    }
    fn boot_prog(&mut self, prog: Prog) {
        unsafe {
            let prog_data = self.get_mut(prog);
            prog_data.state = State::Rdy;
            riscv_utils::write_machine_reg!(prog_data.info.boot_mepc => "mepc");
            crate::println!("\n\n## Starting {:?} ##", prog_data.info.id);
            self.switch(prog);
            clint::set_time_cmp();
            PROG_LIST.unsafe_unlock();
            core::arch::asm!("mret");
        }
    }
    fn get_free_idx(&self) -> usize {
        for (idx, prog) in self.progs.iter().enumerate() {
            if prog.is_none() {
                return idx;
            }
        }
        panic!("No free index for user prog available");
    }
    /// Returns the mut ProgData to a Prog.
    ///
    /// Panics if the ProgData is not found or the option is [None].
    fn get_mut(&mut self, prog: Prog) -> &mut ProgData {
        if let Some(ref mut cur) = self.progs[prog.idx] {
            if cur.info.id == prog.id {
                return cur;
            }
            panic!(
                "Tried to access a user prog: {:?}, at: {}, but a different user prog was found: {:?}",
                prog.id, prog.idx, cur.info.id
            );
        }
        panic!(
            "Tried to access a not existing user prog: {:?}, at: {}",
            prog.id, prog.idx
        );
    }
    /// Returns the ProgData to a Prog.
    ///
    /// Panics if the ProgData is not found or the option is [None].
    fn get(&self, prog: Prog) -> &ProgData {
        if let Some(ref cur) = self.progs[prog.idx] {
            if cur.info.id == prog.id {
                return cur;
            }
            panic!(
                "Tried to access a user prog: {:?}, at: {}, but a different user prog was found: {:?}",
                prog.id, prog.idx, cur.info.id
            );
        }
        panic!(
            "Tried to access a not existing user prog: {:?}, at: {}",
            prog.id, prog.idx
        );
    }
    fn cur_prog_data(&mut self) -> &mut ProgData {
        if let Some(cur) = &mut self.progs[self.cur_prog_idx] {
            return cur;
        }
        panic!("Tried to access current user prog, but none was running");
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Prog {
    idx: usize,
    id: user_prog::Id,
}
impl Prog {
    pub fn set_rdy(&self) {
        PROG_LIST.lock().get_mut(*self).state = State::Rdy;
    }
    pub fn is_blocked(&self, reason: Reason) -> bool {
        PROG_LIST.lock().get(*self).state == State::Blocked(reason)
    }
    pub fn set_blocked(&self, reason: Reason) {
        PROG_LIST.lock().get_mut(*self).state = State::Blocked(reason);
    }
    pub fn increment_mepc(&self) {
        PROG_LIST.lock().get_mut(*self).mepc += 4;
    }
    pub fn id(&self) -> user_prog::Id {
        PROG_LIST.lock().get(*self).info.id
    }
    pub fn prog_info(&self) -> user_prog::Info {
        PROG_LIST.lock().get(*self).info
    }
    pub fn sp(&self) -> usize {
        PROG_LIST.lock().get(*self).sp
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
