#![no_std]
#![no_main]
use system_calls as sys;
use user_shared::*;

#[no_mangle]
extern "C" fn main() {
    let mut i = 0;
    sys::print("\n1: current number is: ");
    while i != 15000000 {
        if i % 1000000 == 0 {
            sys::print_num(i / 1000000);
            sys::print_char(' ');
        }
        i += 1;
    }

    sys::exit();
}
