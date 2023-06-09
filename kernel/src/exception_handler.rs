//! The exception handler.
//! Called from `exception.S` whenever an exception or interrupt occurs.

use crate::{
    hardware::{binary_struct::BinaryStruct, clint, plic, stack::Stack, uart},
    scheduler,
};

use super::sys_call;
use riscv_utils::*;

#[no_mangle]
unsafe extern "C" fn exception_handler(mepc: usize, mcause: usize, sp: usize) -> usize {
    scheduler::save_cur_prog(mepc, sp);
    let mut mcause = BinaryStruct::from(mcause);
    let interrupt = mcause.is_set(63);
    if interrupt {
        mcause.at(63, false);
        handle_interrupt(mcause.into_inner());
    } else {
        handle_exception(mcause.into_inner(), mepc, sp);
    }
    scheduler::restore_cur_prog()
}

unsafe fn handle_interrupt(mcause: usize) {
    match mcause {
        MCAUSE_INTERRUPT_TIMER => {
            let next = scheduler::next();
            scheduler::switch(
                next.expect("No available next user prog. Idle task not implemented"),
            );
            clint::set_time_cmp();
        }
        MCAUSE_INTERRUPT_EXTERN => {
            let irq = plic::read_claim();
            match irq {
                plic::Irq::Uart => {
                    if uart::get_interrupt_cause() == uart::Interrupt::ReceivedDataRdy {
                        if let Some(uart_prog) = uart::read_char_to_buffer() {
                            if uart_prog.is_blocked(scheduler::Reason::Uart) {
                                let char = uart::get_char()
                                    .expect("Char should be present as it was just read");
                                let mut stack = Stack::new(uart_prog.sp());
                                stack.set_ret(char as usize);
                                stack.write();
                                uart_prog.set_rdy();
                                scheduler::switch(uart_prog);
                            }
                        }
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
        MCAUSE_EXCEPTION_IAF => {
            let mtval: usize;
            read_machine_reg!("mtval" => mtval);
            panic!(
                "Instruction access fault in user prog: {:?}, mepc: 0x{:x}, mtval: 0x{:x}",
                scheduler::cur().id(),
                mepc,
                mtval
            );
        }
        MCAUSE_EXCEPTION_II => {
            let mtval: usize;
            read_machine_reg!("mtval" => mtval);
            panic!(
                "Illegal instruction in user prog: {:?}, mepc: 0x{:x}, mtval: 0x{:x}",
                scheduler::cur().id(),
                mepc,
                mtval
            );
        }
        MCAUSE_EXCEPTION_LAF => {
            let mtval: usize;
            read_machine_reg!("mtval" => mtval);
            panic!(
                "Load access fault in user prog: {:?}, mepc: 0x{:x}, mtval: 0x{:x}",
                scheduler::cur().id(),
                mepc,
                mtval
            );
        }
        MCAUSE_EXCEPTION_ECALL => {
            let mut stack = Stack::new(sp);
            let number = stack.a7();
            let param_0 = stack.a0();
            let param_1 = stack.a1();
            if let Some(ret) = sys_call::sys_call(number, param_0, param_1) {
                stack.set_ret(ret);
                stack.write();
            }
        }
        _ => {
            // Unsupported exception
            panic!("Unsupported exception with code: {}", mcause);
        }
    }
}
