//! # 全局属性
//! - `#![no_std]`
//! 禁用标准库
#![no_std]
//!
//! - `#![nomain]`
//! 不使用`main`函数等全部 Rust-level 入口点作为程序入口
#![no_main] 
//! # 一些 unstable 的功能需要在 crate 层级声明后才能使用
//!
//! - `#![feature(alloc_error_handler)]`
//! 使用了全局动态内存分配器，以实现原本标准库中的堆内存分配
#![feature(alloc_error_handler)]
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
mod memory;

extern crate alloc;

// 汇编编写的程序入口
global_asm!(include_str!("entry.asm"));

/// Rust 的入口函数
/// 
/// 在 `_start` 进行一系列准备后，第一个被调用的Rust函数
#[no_mangle]
pub extern "C" fn rust_main() {
    // 初始化各种模块
    interrupt::init();
    memory::init();

    println!("Hello rCore-Tutorial!");

    use alloc::boxed::Box;
    use alloc::vec::Vec;
    let v = Box::new(5);
    assert_eq!(*v, 5);
    core::mem::drop(v);

    let mut vec = Vec::new();
    for i in 0..10000 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10000);
    for (i, value) in vec.into_iter().enumerate() {
        assert_eq!(value, i);
    }
    println!("heap test passed");

    // 这里的 KERNEL_END_ADDRESS 为 ref 类型，需要加*
    // 显示内核使用的结尾地址
    println!("{}", *memory::config::KERNEL_END_ADDRESS);
    
    // 物理页分配
    for _ in 0..2 {
        let frame_0 = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };
        let frame_1 = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };
        println!("{} and {}", frame_0.address(), frame_1.address());
    }

    panic!()
}
