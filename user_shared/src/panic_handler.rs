#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        crate::sys_call::print("Panicked in User!");
    }
}
