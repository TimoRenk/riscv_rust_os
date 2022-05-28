use super::byte::Byte;
use super::register_mapping::RegisterMapping;

//todo catch race condition?!
pub unsafe fn print_string(str: &str) {
    str.chars().for_each(|c| UART.print_char(c));
}
pub unsafe fn print_char(char: char) {
    UART.print_char(char);
}

const UART_BASE_ADDR: usize = 0x1000_0000;
static mut UART: UART = UART {
    register: RegisterMapping::new(UART_BASE_ADDR),
};

struct UART {
    register: RegisterMapping<UartRegister>,
}

impl UART {
    fn print_char(&mut self, char: char) {
        let register = self.register.get();
        while !register.5.is_set(5) {}
        register.0.write(char as u8);
    }
}

/// offset_0: RBR, THR | DLL
/// offset_1: IER | DLM
/// offset_2: IIR, FCR
/// offset_3: LCR
/// offset_4: MCR
/// offset_5: LSR
/// offset_6: MSR
/// offset_7: SCR
#[repr(C)]
struct UartRegister(Byte, Byte, Byte, Byte, Byte, Byte, Byte, Byte);
