use riscv::register::{sstatus::Sstatus, scause::Scause};

#[repr(C)]
#[derive(Debug)]
pub struct Context {  // 原来程序正在执行所在的上下文
    pub x: [usize; 32],   // 通用寄存器
    pub sstatus: Sstatus,
    pub sepc: usize,
}