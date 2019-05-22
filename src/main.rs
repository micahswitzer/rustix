#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustix::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rustix::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
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
