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
    use os::memory;
    use x86_64::{structures::paging::Translate, VirtAddr};
    let phys_mem_offset = VirtAddr::new(_boot_info.physical_memory_offset);
    let mapper = unsafe { memory::init(phys_mem_offset) };
    let addresses = [
        0xb8000,                           //VGA buffer
        0x201008,                          // a code page
        0x0100_0020_1a10,                  //a stack page
        _boot_info.physical_memory_offset, // virtual address mapped to physical address 0
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{virt:?} -> {:?}", phys.unwrap());
    }
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
