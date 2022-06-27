#![no_std]
#![no_main]
mod asm;
mod panic_handler;
mod system_calls;
use system_calls as sys;

#[no_mangle]
extern "C" fn main() {
    sys::print_num(2);
    sys::exit();
}
