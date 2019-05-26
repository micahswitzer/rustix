#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustix::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use rustix::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World!");

    rustix::init();

    #[cfg(test)]
    test_main();

    // remind the haters that we didn't crash
    println!("No crashes today!");

    // go to sleep
    rustix::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rustix::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustix::test_panic_handler(info);
}
