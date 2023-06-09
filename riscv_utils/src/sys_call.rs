use enum_matching::EnumTryFrom;

/// System calls enum.
/// Required for kernel and user prog to sync the system call index.
#[derive(EnumTryFrom)]
pub enum SysCall {
    PrintString,
    PrintChar,
    PrintNum,
    GetChar,
    UartOpen,
    UartClose,
    Yield = 23,
    Exit = 42,
}
