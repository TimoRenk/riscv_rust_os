#![no_std]
#![no_main]

mod asm;
mod kernel;
mod user;

//todo!
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn kernel_setup() {
    kernel::setup();
    kernel::hello_world();
}
