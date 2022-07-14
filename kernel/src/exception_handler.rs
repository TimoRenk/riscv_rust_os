use crate::{
    hardware::{binary_struct::BinaryStruct, clint, memory_mapping::MemoryMapping, plic, uart},
    macros::print,
    scheduler,
};

use super::system_calls;
use riscv_utils::*;

#[no_mangle]
unsafe extern "C" fn exception_handler(mepc: usize, mcause: usize, sp: usize) -> usize {
    scheduler::save_cur_prog(mepc, sp);
    let mut mcause = BinaryStruct::from(mcause);
    let interrupt = mcause.is_set(63);
    if interrupt {
        mcause.at(63, false);
        handle_interrupt(mcause.get());
    } else {
        handle_exception(mcause.get(), mepc, sp);
    }
    let sp = scheduler::restore_cur_prog();
    return sp;
}

unsafe fn handle_interrupt(mcause: usize) {
    match mcause {
        7 => {
            // Timer interrupt
            let next = scheduler::next();
            scheduler::switch(
                next.expect("No available next user prog. Idle task not implemented"),
            );
            clint::set_time_cmp();
        }
        11 => {
            // Extern interrupt
            let irq = plic::read_claim();
            match irq {
                plic::IRQ::Uart => {
                    if uart::get_interrupt_cause() == uart::UartInterrupt::ReceivedDataRdy {
                        let char = uart::read_char();
                        print!("{}", char);
                    } else {
                        panic!(
                            "Unsupported UART interrupt with code: {:?}",
                            uart::get_interrupt_cause()
                        );
                    }
                }
            }
            plic::write_complete(irq);
        }
        _ => {
            panic!("Unsupported interrupt with code: {}", mcause);
        }
    }
}

unsafe fn handle_exception(mcause: usize, mepc: usize, sp: usize) {
    match mcause {
        1 => {
            // Instruction access fault
            let mtval: usize;
            read_machine_reg!("mtval" => mtval);
            panic!(
                "Instruction access fault in user prog: {:?}, mepc: 0x{:x}, mtval: 0x{:x}",
                scheduler::cur().id(),
                mepc,
                mtval
            );
        }
        5 => {
            // Load access fault
            let mtval: usize;
            read_machine_reg!("mtval" => mtval);
            panic!(
                "Load access fault in user prog: {:?}, mepc: 0x{:x}, mtval: 0x{:x}",
                scheduler::cur().id(),
                mepc,
                mtval
            );
        }
        8 => {
            // Ecall from user-mode
            let mut mem_stack = MemoryMapping::new(sp as usize);
            let mut stack: Stack = mem_stack.read();
            let number = stack[16]; // a7
            let param_0 = stack[9]; // a0
            let param_1 = stack[10]; // a1
            let ret_val = system_calls::syscall(number, param_0, param_1);
            stack[9] = ret_val;
            mem_stack.write(stack);
        }
        _ => {
            // Unsupported exception
            panic!("Unsupported exception with code: {}", mcause);
        }
    }
}
type Stack = [usize; 32];
