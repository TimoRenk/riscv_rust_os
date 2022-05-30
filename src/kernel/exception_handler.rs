use core::arch::asm;

use super::hardware::memory_mapping::MemoryMapping;
use super::hardware::riscv::{self, Register};
use super::hardware::uart;
use super::system_calls;
#[no_mangle]
extern "C" fn exception_handler() {
    let number;
    let param_1;
    let param_2;
    unsafe {
        asm!("add {}, a0, zero", out(reg) param_1);
        asm!("add {}, a1, zero", out(reg) param_2);
        asm!("add {}, a7, zero", out(reg) number);
        let return_value = 0;
        match number {
            system_calls::PRINT_CHAR => print_char(param_1),
            system_calls::PRINT_STRING => print_string(param_1, param_2),

            _ => {
                uart::print_string("System call not implemented: ");
                uart::print_char(number as u8 as char)
            }
        }
        asm!("add a0, {}, zero", in(reg) return_value);
    }
    let mepc = riscv::read_register(Register::MEPC);
    riscv::write_register(Register::MEPC, mepc + 4);
}

fn print_string(str_ptr: u64, size: u64) {
    unsafe {
        let mut str_ptr = str_ptr as *const u8;
        for _ in 0..size {
            let char = *MemoryMapping::<u8>::new(str_ptr as usize).get();
            uart::print_char(char as char);
            str_ptr = str_ptr.add(1);
        }
    }
}

fn print_char(char: u64) {
    unsafe {
        uart::print_char(char as u8 as char);
    }
}
