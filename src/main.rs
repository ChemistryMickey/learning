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
    use os::memory::active_level_4_table;
    use x86_64::{structures::paging::PageTable, VirtAddr};

    let phys_mem_offset = VirtAddr::new(_boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {i}: {entry:#?}");

            // Get physical address from entry
            let phys = entry.frame().unwrap().start_address();
            let virt = phys.as_u64() + _boot_info.physical_memory_offset;
            let ptr = VirtAddr::new(virt).as_mut_ptr();
            let l3_table: &PageTable = unsafe { &*ptr };

            // print non-empty entries of the L3 table
            for (i, entry) in l3_table.iter().enumerate() {
                if !entry.is_unused() {
                    println!("          L3 Entry {i}: {entry:#?}");
                }
            }
        }
    }
    //==================
    #[cfg(test)]
    test_main();

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
