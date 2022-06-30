#![no_std]
#![no_main]

//todo
mod asm;
mod exception_handler;
mod hardware;
mod panic_handler;
mod setup;
mod system_calls;
mod user_progs;

//todo implement shutdown
fn _shutdown() {}

#[no_mangle]
unsafe extern "C" fn kernel_setup() {
    setup::setup();
    // switch to user mode (configured in mstatus) and jump to address in mepc CSR -> main().
    user_progs::switch_prog(user_progs::Progs::User1);
    core::arch::asm!("mret");
}
