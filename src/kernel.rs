//todo
mod asm;
mod exception_handler;
mod hardware;
pub mod system_calls;
pub use hardware::setup::setup;

//todo implement shutdown
fn _shutdown() {}
