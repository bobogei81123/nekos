#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::{exit_qemu, println, QemuExitCode};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

fn should_fail() {
    println!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}
