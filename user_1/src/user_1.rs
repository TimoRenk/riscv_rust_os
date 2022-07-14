#![no_std]
#![no_main]
use sys_call as sys;
use user_shared::*;

#[no_mangle]
extern "C" fn main() {
    if sys::get_char().is_some() {
        sys::print("\nu1: Is not allowed to get a char!");
    }
    if sys::uart_close() {
        sys::print("\nu1: Is not allowed to close uart!");
    }
    for i in 0..10000000 {
        if i % 1000000 == 0 {
            sys::print("\n");
            sys::print_num(i / 1000000);
        }
    }
    while !sys::uart_open() {}
    sys::print("\nuart is open!");
    for i in 10000000..20000000 {
        if i % 1000000 == 0 {
            sys::print("\n");
            sys::print_num(i / 1000000);
        }
    }
    if !sys::uart_open() {
        sys::print("\nu1: Uart should be open!");
    }
    for _ in 0..7 {
        if let Some(char) = sys::get_char() {
            sys::print("\n");
            sys::print_char(char);
        }
    }
    if !sys::uart_close() {
        sys::print("\nu1: should be allowed to close uart!");
    }
    sys::exit();
}
