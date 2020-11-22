use crate::thread::thread::Context;

global_asm!(include_str!("thread_switch.s"));
extern "C" {
    pub(super) fn x86_64_thread_switch(prev: *mut Context, next: *const Context);
}
