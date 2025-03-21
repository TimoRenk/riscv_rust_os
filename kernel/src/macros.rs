#[allow(unused)]
macro_rules! print {
    ($($arg:tt)*) => {
        use core::fmt::Write;
        write!(crate::hardware::uart::UART.lock(), $($arg)*).ok()}
}
#[allow(unused)]
pub(crate) use print;

#[allow(unused)]
macro_rules! println {
    ($($arg:tt)*) => {
        crate::hardware::uart::print_char('\n');
        use core::fmt::Write;
        write!(crate::hardware::uart::UART.lock(), $($arg)*).ok()}
}
#[allow(unused)]
pub(crate) use println;
