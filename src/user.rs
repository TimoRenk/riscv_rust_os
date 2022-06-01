mod syscall;
use syscall as sys;

pub fn main() -> ! {
    sys::println("Hello Bamberg!");
    crate::kernel::system_calls::print_char('Z');
    crate::kernel::system_calls::print_string("Hello World!");
    loop {}
}
