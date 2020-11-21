//! 内存管理模块
//!
//! 负责空间分配、虚拟地址映射

pub mod heap;
pub mod config;
pub mod address;
pub mod frame;
pub mod range;

pub use {address::*, config::*, frame::FRAME_ALLOCATOR, range::Range};

/// 一个缩写，模块中一些函数会使用
pub type MemoryResult<T> = Result<T, &'static str>;

/// 初始化内存管理相关的子模块
///
/// - [`heap::init`]
pub fn init() {
    heap::init();
    println!("mod memory initialized");
}