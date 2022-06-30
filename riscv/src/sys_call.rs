pub enum SysCall {
    PrintString,
    PrintChar,
    PrintNum,
    GetChar,
    Yield = 23,
    Exit = 42,
}