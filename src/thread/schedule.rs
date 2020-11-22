use crate::thread::{
    thread::Context,
    thread_switch::x86_64_thread_switch,
};
use core::{
    ptr::{self, NonNull},
    cell::UnsafeCell,
};
use alloc::{
    boxed::Box,
    collections::linked_list::LinkedList,
};
use super::thread::Thread;
use x86_64::{
    registers::model_specific::GsBase,
    VirtAddr,
};

struct Scheduler {
    current_thread: NonNull<Thread>,
    queue: LinkedList<NonNull<Thread>>,
}

impl Scheduler {
    fn schedule(&mut self) {
        if self.queue.is_empty() {
            return;
        }

        unsafe {
            let mut prev = self.current_thread;
            let next = self.queue.pop_front().unwrap();

            self.queue.push_back(prev);
            self.current_thread = next;

            thread_switch(&mut prev.as_mut().context, &next.as_ref().context);
        }
    }

    fn push(&mut self, thread: &'static mut Thread) {
        self.queue.push_back(NonNull::from(thread));
    }
}

fn thread_switch(prev: &mut Context, next: &Context) {
    unsafe {
        x86_64_thread_switch(prev as *mut Context, next as *const Context);
    }
}

pub fn start() {
    let init_thread = Box::leak(Box::new(Thread::new_init()));
    let scheduler = Box::new(Scheduler {
        current_thread: NonNull::from(init_thread),
        queue: LinkedList::new(),
    });

    GsBase::write(VirtAddr::from_ptr(Box::leak(scheduler) as *const Scheduler));
}

fn scheduler() -> &'static mut Scheduler {
    unsafe {
        GsBase::read().as_mut_ptr::<Scheduler>().as_mut().expect("ptr is null")
    }
}


pub fn push(thread: &'static mut Thread) {
    scheduler().push(thread);
}

pub fn schedule() {
    scheduler().schedule();
}
