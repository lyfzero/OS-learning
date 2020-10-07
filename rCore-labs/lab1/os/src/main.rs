//! # 全局属性
//! - `#![no_std]`
//! 禁用标准库
#![no_std]
//!
//! - `#![nomain]`
//! 不使用`main`函数等全部 Rust-level 入口点作为程序入口
#![no_main] 
//! # 一些 unstable 的功能需要在 crate 层级声明后才能使用
//! - `#![feature(llvm_asm)]`
//! 内嵌汇编
#![feature(llvm_asm)]
//!
//! - `#![feature(global_asm)]`
//! 内嵌整个汇编文件
#![feature(global_asm)]
//!
//! - `#![feature(panic_info_message)]`
//! panic! 时，获取其中的信息并打印
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod interrupt;
mod panic;
mod sbi;

// 汇编编写的程序入口
global_asm!(include_str!("entry.asm"));

/// Rust 的入口函数
/// 
/// 在 `_start` 进行一系列准备后，第一个被调用的Rust函数
#[no_mangle]
pub extern "C" fn rust_main() {
    // 初始化各种模块
    interrupt::init();

    println!("Hello rCore-Tutorial!");

    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    };

    panic!("end of rust_main");
}
