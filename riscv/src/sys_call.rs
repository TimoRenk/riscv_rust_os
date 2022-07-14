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
