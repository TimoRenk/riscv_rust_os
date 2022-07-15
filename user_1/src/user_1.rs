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
    for i in 1..5000001 {
        if i % 1000000 == 0 {
            sys::print("\n");
            sys::print_num(i / 1000000);
        }
    }
    if sys::uart_open() {
        sys::print("\nuart is open!");
        for i in 5000001..10000001 {
            if i % 1000000 == 0 {
                sys::print("\n");
                sys::print_num(i / 1000000);
            }
        }
        let mut text = ['\r'; 50];
        for i in 0..text.len() {
            let char = sys::get_char().unwrap();
            if char == '\r' {
                break;
            }
            text[i] = char;
        }
        sys::print_char('\n');
        for char in text {
            if char == '\r' {
                break;
            }
            sys::print_char(char);
        }
        if !sys::uart_open() {
            sys::print("\nu1: Uart should be open!");
        }
        if !sys::uart_close() {
            sys::print("\nu1: should be allowed to close uart!");
        }
    }
    sys::exit();
}
