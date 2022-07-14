#![no_std]
#![no_main]

mod asm;
mod exception_handler;
mod hardware;
mod macros;
mod panic_handler;
mod scheduler;
mod setup;
mod system_calls;
mod user_prog;

pub(crate) use macros::*;

//todo implement shutdown
fn _shutdown() {}

#[no_mangle]
unsafe extern "C" fn kernel_setup() {
    setup::setup();
    // switch to user mode (configured in mstatus) and jump to address in mepc CSR -> main().
    let user1 = scheduler::init_prog(user_prog::USER1);
    scheduler::init_prog(user_prog::USER2);
    scheduler::boot_prog(user1);
}
