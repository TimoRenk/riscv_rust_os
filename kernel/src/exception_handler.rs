use core::arch::asm;

use super::hardware::uart;
use super::system_calls::*;
use riscv_utils::*;

#[no_mangle]
unsafe extern "C" fn exception_handler() {
    let number: u64;
    let param_0;
    let param_1;
    read_function_reg!(
        "a7" => number,
        "a1" => param_1,
        "a0" => param_0
    );
    let mut return_value = 0;
    match SystemCall::try_from(number) {
        Ok(number) => match number {
            SystemCall::PrintString => print_string(param_0, param_1),
            SystemCall::PrintChar => print_char(param_0),
            SystemCall::GetChar => return_value = get_char() as u64,
            SystemCall::PrintNum => print_num(param_0),
            SystemCall::Exit => exit(),
        },
        Err(error) => {
            uart::print_string(error.message);
            uart::print_num(error.syscall);
        }
    }
    let mepc: u64;
    read_machine_reg!("mepc" => mepc);
    write_machine_reg!(mepc + 4 => "mepc");
    write_function_reg!(return_value => "a0");
}
