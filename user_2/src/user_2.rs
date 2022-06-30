#![no_std]
#![no_main]
use system_calls as sys;
use user_shared::*;

#[no_mangle]
extern "C" fn main() {
    for i in 0..7000000 {
        if i % 1000000 == 0 {
            sys::print("\n2: current number is: ");
            sys::print_num(i);
            sys::sys_yield();
        }
    }
    sys::exit();
}
