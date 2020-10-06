//! # global attributes
//! - #![no_std]
//! disable standard library
#![no_std]

//!
//! - `#![no_main]`
//! do not use all rust-level entry points such as main function as program entry
#![no_main]
//!
//!
//!
#![feature(llvm_asm)]

//!
//! - `#[feature(global_asm)]`
//!
#![feature(global_asm)]

//
global_asm!(include_Str!("entry.asm"));

use core::panic::PanicInfo;

/// call this function when panic 
/// dead loop
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
/// 在屏幕上输出一个字符，目前我们先不用了解其实现原理
pub fn console_putchar(ch: u8) {
    let _ret: usize;
    let arg0: usize = ch as usize;
    let arg1: usize = 0;
    let arg2: usize = 0;
    let which: usize = 1;
    unsafe {
        llvm_asm!("ecall"
             : "={x10}" (_ret)
             : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which)
             : "memory"
             : "volatile"
        );
    }
}


/// override the start function in crt0
/// dead loop
// #[no_mangle]
// pub extern "C" fn _start() -> ! {
//     loop {}
// }

/// Rust 的入口函数
///
/// 在 `_start` 为我们进行了一系列准备之后，这是第一个被调用的 Rust 函数
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // 在屏幕上输出 "OK\n" ，随后进入死循环
    console_putchar(b'O');
    console_putchar(b'K');
    console_putchar(b'\n');

    loop {}
}