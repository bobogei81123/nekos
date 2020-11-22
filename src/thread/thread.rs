use x86_64::VirtAddr;
use crate::memory;

/// Context that is saved during a context switch.
#[derive(Debug, Default)]
#[repr(C)]
pub(super) struct Context {
    rsp: u64,
    rbp: u64,
    rbx: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
}

#[derive(Debug)]
pub struct Thread {
    id: ThreadId,
    pub(super) context: Context,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ThreadId(u64);

impl ThreadId {
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    fn new() -> Self {
        use core::sync::atomic::{AtomicU64, Ordering};
        static NEXT_THREAD_ID: AtomicU64 = AtomicU64::new(2);
        ThreadId(NEXT_THREAD_ID.fetch_add(1, Ordering::Relaxed))
    }
}

impl Thread {
    pub fn new_init() -> Self {
        Thread {
            id: ThreadId(1),
            context: Default::default(),
        }
    }

    /// Create a new thread that execute the function `func`.
    pub fn new(func: fn()) -> Self {

        let func = func as *const fn();
        let mut stack_pointer = memory::page_alloc::PAGE_ALLOCATOR
            .lock()
            .alloc_pages(2)
            .end
            .start_address();

        stack_pointer -= 8usize;
        unsafe {
            stack_pointer.as_mut_ptr::<*const fn()>().write(func);
        }
        stack_pointer -= 8usize;

        let mut thread = Thread {
            id: ThreadId::new(),
            context: Context::default(),
        };
        thread.context.rsp = stack_pointer.as_u64();
        thread
    }
}
