#[allow(unused)]
macro_rules! print {
    ($($arg:tt)*) => {
        use core::fmt::Write;
        write!(crate::hardware::uart::get_uart(), $($arg)*).ok()}
}
#[allow(unused)]
pub(crate) use print;

#[allow(unused)]
macro_rules! println {
    ($($arg:tt)*) => {
        crate::hardware::uart::print_char('\n');
        use core::fmt::Write;
        write!(crate::hardware::uart::get_uart(), $($arg)*).ok()}
}
#[allow(unused)]
pub(crate) use println;

#[allow(unused)]
macro_rules! enum_matching {
    ($num:ident: $($enum:expr), +) => {
        $(if $num == $enum as usize {
            return $enum;
        }) +
    };
}

#[allow(unused)]
pub(crate) use enum_matching;
