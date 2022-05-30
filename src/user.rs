pub fn main() -> ! {
    crate::kernel::system_calls::print_char('Z');
    crate::kernel::system_calls::print_string("Hello World!");
    loop {}
}
