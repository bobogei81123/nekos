use core::{
    cell::UnsafeCell,
    mem::{self, MaybeUninit},
    ops::{Deref, DerefMut},
};

pub struct UnsafeOnceCell<T>(MaybeUninit<UnsafeCell<T>>);

impl<T> UnsafeOnceCell<T>
where T: Sized {
    pub const unsafe fn new() -> Self {
        UnsafeOnceCell(MaybeUninit::uninit())
    }

    pub unsafe fn init(&self, val: T) {
        UnsafeCell::raw_get(self.0.as_ptr()).write(val);
    }
}

unsafe impl<T> Sync for UnsafeOnceCell<T> {}

impl<T> Deref for UnsafeOnceCell<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.0.get_ref().get() }
    }
}
