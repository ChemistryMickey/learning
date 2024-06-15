#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use os::{
    allocator,
    memory::{self, BootInfoFrameAllocator},
};
use os::{
    println,
    task::{executor::Executor, keyboard, Task},
};
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    os::init();

    // setup heap
    let phys_mem_offset = VirtAddr::new(_boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&_boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // Allow tests to run
    #[cfg(test)]
    test_main();
    // ================= CODE GO HERE
    let mut exector = Executor::new();
    exector.spawn(Task::new(example_task()));
    exector.spawn(Task::new(keyboard::print_keypresses()));

    // This never exits
    exector.run();
    //==================
}

async fn async_number() -> u32 {
    return 42;
}

async fn example_task() {
    let num = async_number().await;

    println!("async number: {num}");
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
