// src/main.rs

#![no_std] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

use core::panic::PanicInfo;
use hacker_os::println;

/// This function is the entry point, since the linker looks for a function
/// named `_start` by default.
#[cfg(not(test))]
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
 
    use x86_64::structures::paging::PageTable;

    let level_4_table_ptr = 0xffff_ffff_ffff_f000 as *const PageTable;
    let level_4_table = unsafe {&*level_4_table_ptr};
    for i in 0..10 {
        println!("Entry {}: {:?}", i, level_4_table[i]);
    }

 
    println!("Hello World{}", "!");

    hacker_os::gdt::init();
    hacker_os::interrupts::init_idt();
    unsafe { hacker_os::interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();  

 
    println!("It did not crash!");

    hacker_os::hlt_loop();

}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hacker_os::hlt_loop();
}
