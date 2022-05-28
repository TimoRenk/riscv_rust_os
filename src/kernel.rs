mod exception_handler;
//todo
mod hardware;
mod system_calls;
use super::user;
pub use hardware::setup::setup;

//todo remove
#[no_mangle]
extern "C" fn kernel_main() -> ! {
    user::hello_world();
    loop {}
}
//todo implement shutdown
fn _shutdown() {}

pub fn hello_world() {
    unsafe {
        hardware::uart::print_string("Finally!");
    }
}
