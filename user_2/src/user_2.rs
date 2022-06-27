#![no_std]
#![no_main]
use system_calls as sys;
use user_shared::*;

#[no_mangle]
extern "C" fn main() {
    sys::print_num(2);
    sys::exit();
}
