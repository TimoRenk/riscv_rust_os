#![no_std]
#![no_main]

core::arch::global_asm!(include_str!("asm/boot.S"));

pub const UART_BASE_ADDR: usize = 0x1000_0000;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn kernel_main() {
    print_string("Hello from Rust! :D");
}

fn print_char(c: char) {
    let ptr = UART_BASE_ADDR as *mut u8;
    unsafe {
        while ptr.add(5).read_volatile() & (1 << 5) == 0 {}
        ptr.add(0).write_volatile(c as u8);
    }
}

fn print_string(str: &str) {
    str.chars().for_each(|c| print_char(c));
}
