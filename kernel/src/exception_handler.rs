use crate::{
    hardware::{binary_struct::BinaryStruct, clint, plic, stack::Stack, uart},
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
                    if uart::get_interrupt_cause() == uart::Interrupt::ReceivedDataRdy {
                        if let Some(uart_prog) = uart::read_char() {
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
            let mut stack = Stack::new(sp);
            let number = stack.a7();
            let param_0 = stack.a0();
            let param_1 = stack.a1();
            if let Some(ret) = system_calls::syscall(number, param_0, param_1) {
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
