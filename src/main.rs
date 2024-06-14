#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use os::println;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    os::init();

    // ================= CODE GO HERE
    //==================
    #[cfg(test)]
    test_main();

    println!("It didn't crash!");
    os::hlt_loop();

    // #[allow(clippy::empty_loop)]
    // loop {
    //     use os::print;
    //     print!("-");
    //     for _ in 0..10000 {}
    // }
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}

#[test_case]
fn trivial_test() {
    assert_eq!(1, 1);
}
