use crate::hardware::uart::UART;
use crate::print;

#[panic_handler]
unsafe fn panic(info: &core::panic::PanicInfo) -> ! {
    UART.unsafe_unlock();
    print!("\n\n\n### System Crash ###\n{}", info);
    loop {}
}
