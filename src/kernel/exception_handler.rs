use super::hardware::uart;
fn _print_char(char: char) {
    uart::print_char(char);
}

fn _print_string(str: &str) {
    uart::print_string(str);
}
