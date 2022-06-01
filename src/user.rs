mod syscall;
use syscall as sys;

pub fn main() -> ! {
    sys::println("Hello Bamberg!");
    sys::print_num(000004200000);
    loop {
        let input = sys::get_char();
        if input == 's' {
            continue;
        } else if input == 'X' {
            break;
        }
        sys::print_char(input);
    }
    sys::print("\n\n### END OF MAIN ###");
    loop {}
}
