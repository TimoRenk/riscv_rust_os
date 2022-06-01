#![no_std]
#![no_main]

mod kernel;
mod riscv;
mod user;

//todo!
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
unsafe extern "C" fn kernel_setup() {
    kernel::setup();
    //todo! Not reached due to mret
}
