#![no_std]
#![no_main]
use system_calls as sys;
use user_shared::*;

#[no_mangle]
extern "C" fn main() {
    for i in 0..10000000 {
        if i % 1000000 == 0 {
            sys::print("\n1: current number is: ");
            sys::print_num(i);
            sys::sys_yield();
        }
    }
    sys::exit();
}
