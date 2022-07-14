#![no_std]
#![no_main]
use sys_call as sys;
use user_shared::*;

#[no_mangle]
extern "C" fn main() {
    if sys::get_char().is_some() {
        sys::print("\nu2: Is not allowed to get a char!");
    }
    if sys::uart_close() {
        sys::print("\nu2: Is not allowed to close uart!");
    }
    for i in 0..5000000 {
        if i % 1000000 == 0 {
            sys::print("\n        ");
            sys::print_num(i / 1000000);
        }
    }
    while !sys::uart_open() {}
    sys::print("\n        uart is open!");
    for i in 5000000..7000000 {
        if i % 1000000 == 0 {
            sys::print("\n        ");
            sys::print_num(i / 1000000);
        }
    }
    if !sys::uart_open() {
        sys::print("\nu2: Uart should be open!");
    }
    if let Some(char) = sys::get_char() {
        sys::print("\n        ");
        sys::print_char(char);
    }
    if !sys::uart_close() {
        sys::print("\nu2: should be allowed to close uart!");
    }
    sys::exit();
}
