pub mod thread;
pub mod thread_switch;
pub mod schedule;

pub fn init() {
    schedule::start();
}
