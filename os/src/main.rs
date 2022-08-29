#![no_main]
#![no_std]
#![feature(panic_info_message)]

use core::arch::global_asm;
use crate::sbi::shutdown;

#[macro_use]
mod console;
mod lang_items;
mod sbi;


global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack();
        fn boot_stack_top();
    }

    clear_bss();
    error!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    warn!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    debug!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    trace!(".boot_stack [{:#x}, {:#x})", boot_stack as usize, boot_stack_top as usize);

    println!("hello world!");
    shutdown();
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}