mod exception_handler;
//todo
pub mod hardware;
mod system_call;
use crate::user;

#[no_mangle]
extern "C" fn kernel_main() -> ! {
    user::hello_world();
    loop {}
}
//todo implement shutdown
fn _shutdown() {}
