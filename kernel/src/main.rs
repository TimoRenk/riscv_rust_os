#![no_std]
#![no_main]

mod asm;
mod exception_handler;
mod hardware;
mod panic_handler;
mod setup;
mod system_calls;
mod user_prog;

//todo implement shutdown
fn _shutdown() {}

#[no_mangle]
unsafe extern "C" fn kernel_setup() {
    setup::setup();
    // switch to user mode (configured in mstatus) and jump to address in mepc CSR -> main().
    user_prog::start_prog(user_prog::get());
}

#[allow(unused)]
macro_rules! print {
    ($($arg:tt)*) => {
        use core::fmt::Write;
        write!(crate::hardware::uart::get_uart(), $($arg)*).ok()}
}
#[allow(unused)]
macro_rules! println {
    ($($arg:tt)*) => {
        crate::hardware::uart::print_char('\n');
        use core::fmt::Write;
        write!(crate::hardware::uart::get_uart(), $($arg)*).ok()}
}
#[allow(unused)]
pub(crate) use print;
#[allow(unused)]
pub(crate) use println;
