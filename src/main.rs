#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(llvm_asm)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
#[macro_use]
extern crate rust_os;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rust_os::memory;
use rust_os::thread::{
    self,
    thread::Thread,
};
use rust_os::memory::allocator;
use x86_64::VirtAddr;
use alloc::boxed::Box;

entry_point!(kernel_main);

fn hello() {
    x86_64::instructions::interrupts::enable();
    println!("Thread switch success!");
    for i in 1..100 {
        println!("{}\n", i);
        x86_64::instructions::hlt();
    }
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {

    println!("Hello World{}", "!");
    println!("Boot = {:#?}", boot_info);
    println!("&Boot = {:p}", boot_info);

    rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    unsafe {
        memory::mapper::init(phys_mem_offset);
        memory::frame::init(&boot_info.memory_map);
        memory::page_alloc::init();
    }

    allocator::init_heap().expect("heap initialization failed");

    #[cfg(test)]
    test_main();

    thread::init();
    println!("Init thread\n");

    for i in 1..10 {
        let mut stack = memory::page_alloc::PAGE_ALLOCATOR
            .lock()
            .alloc_pages(10)
            .end
            .start_address();
        let thread = Box::leak(Box::new(Thread::new(hello)));
        thread::schedule::push(thread);
    }

    use rust_os::interrupts;
    interrupts::enable_thread_switch();

    rust_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}
