# lab0

## 学到了什么

- 怎样移除Rust程序对OS的依赖
- 怎样将程序目标平台设置为RISC-V
- 怎样在RISC-V指令集的裸机上执行Rust程序
- 内核镜像是什么，怎样生成，如何调整代码的内存布局，并在QEMU模拟器启动
- QEMU模拟器是什么
- 怎样封装SBI接口（输出、关机等）
- SBI是什么





工具链版本（rust-toolchain）：创建在工作目录中，设置项目的Rust工具版本（nightly）防止更新后程序崩溃



创建了os项目（可执行文件main.rc）



禁用标准库：

- why：项目链接标准库std依赖OS，但是我们希望脱离OS
- how：`#![no_std]`
  - 这是什么语法？说明了什么？
  - 全局属性



复习一下Rust语法：

- 注释（三种）
  - `\\`
  - `\\\`：文档注释，可以通过xxx生成文档
- 宏`println!`
  - 属于std，以来OS标准输出
  - 怎样写不依赖OS的实现？
- panic_handler
  - 这个是什么呢？
  - 某个函数（默认在std中），在程序发生panic时调用
  - 依赖OS特殊的文件描述符
    - 文件描述符是什么？
  - 怎样实现不依赖OS？
- Rust的panic机制需要复习一下
- core库是些什么？
  - 不需要OS的支持
- use声明的语法复习一下
- `#[xxx]`是什么语法？
  - `#![no_main]`：不用常规的入口点
  - `#![no_std]`：不用std标准库
  - `#[no_mangle]`：禁用编译器期间的名称重整(name mangling)，确保生成名为`_start`的函数 
    - 否则会为了实现函数重载生成散列化的函数名
  - `#[panic_handler]`
- `PanicInfo`是什么类型？
  - 包含了panic发生的文件名、代码行数、可选的错误信息
  - 发散函数
    - 返回类型为Never类型(`!`)
- 语义项是什么（language item）
  - 编译器内部所需的特殊函数或类型
- eh_personality
  - eh: Exception Handling
    - 标记函数用来实现**堆栈展开**处理功能的语义项
    - 堆栈展开是什么？
      - 异常点回溯直到捕获或终止
      - 沿着调用栈回溯，回收每个caller中的局部变量，避免造成捕获异常并恢复后的内存溢出
  - 禁用：将dev和release配置的panic处理策略设为直接终止（`abort`）
    - 直接调用`panic_handler`
    - 写在`Cargo.toml`文件中

- 什么是运行时系统
  - main函数不是实际执行的第一个函数
  - Rust程序（链接了标准库）
    - 跳转到c语言运行时环境的`crt0`，设置C语言运行需要的环境
    - 跳转到Rust运行时环境的入口点（被`start`语义项标记）设置环境
    - 调用`main`函数进入主程序
  - 怎样重写入口点`_start`
    - 什么是第一条指令，POST（自检），启动代码（Bootloader）
    - 怎样设置内核的运行环境
    - RISC-V下汇编怎么写？
- RISC-V 64的特权级
  - U Mode(user/application)
  - S Mode(Supervisor)
  - M Mode(machine)
  - 怎样修改
- FFI（语言交互接口）语法
  - `extern "C"`：表示为C函数
- Linux的可执行文件格式？
- 链接错误
- 怎样选择编译为裸机目标，不链接任何运行时环境
  - 选一个底层没有操作系统的运行环境（裸机）
    - 目标三元组：`riscv64imac-unknown-none-elf`
    - 怎样编译呢：`rustup target add xxx(三元组)`
    - 怎样构建程序：build指定target为该三元组
    - 编译结果放在：target下对应环境的debug文件夹
      - 将其配置在 config文件中(`.cargo/config`)
      - 怎样写配置文件？（格式、位置）https://doc.rust-lang.org/cargo/reference/config.html
- 宿主系统是什么
- 目标三元组
  - `<arch><sub>-<vendor>-<sys>-<abi>`（CPU架构、供应商、操作系统、二进制接口）
  - 怎样查看：`rustc --version --verbose`
- 内核镜像是什么？
- bitnutils：命令行工具集
  - 怎样使用
  - objdump
    - -x：查看元信息
      - start address
      - sections
      - SYMBOL TABLE
      - Program Header
        - off
        - vaddr
        - paddr
        - align
        - filesz
        - memsz
        - flags(r,w,x)
    - -d：反汇编
  - objcopy
    - 从elf格式可执行文件生成内核镜像
    - --strip-all：丢弃所有符号表及调试信息
    - -O binary：输出为二进制文件
  - 等工具
- `file xxx(某文件)`：查看文件类型
  - 可执行文件（位、格式、可执行、架构、链接方式、符号表信息）
- 怎样生成镜像
- 内核镜像是什么？
  - kernel.bin
- 怎样调整内存布局
  - 对普通用户程序，代码和数据放在低地址
  - OS内核，放在高地质
    - QEMU模拟的RISC-V中，DRAM内存的物理地址从-X80000000开始，大小128MB
      - qemu/hw/riscv/virt.c：VIRT——DRAM
  - 内存布局
    - 分段
      - .text
      - .rodata
      - .data
      - .bss
      - Heap：运行过程中内存的动态分配
      - Stack
    - 改变链接地址，怎样写链接脚本？https://sourceware.org/binutils/docs/ld/Scripts.html
      - linker.ld
      - 制定架构
      - 入口地址
      - 怎样使用链接脚本
  - OpenSBIhttps://github.com/riscv/opensbi
- makefile怎么写
  - 有一个坑，`make run`的时候出现错误：
    - `Makefile:16: *** 遗漏分割符（null）`
    - 命令行未从tab开始，将每个命令前面的空格全部改为 tab就可以了；
    - 或者反斜线后多了空格
    - （用vscode的高亮看的很清楚）

- 怎样进行接口封装和代码整理
  - SBI是什么
    - 底层系统服务接口，S mode 和 M mode执行环境间的标准接口约定
    - ecall
      - 指定SBI调用的编号，传递参数
- calling convention是什么https://en.wikipedia.org/wiki/Calling_convention
  - https://riscv.org/wp-content/uploads/2015/01/riscv-calling.pdf
  - 
- 怎样实现rust代码与汇编代码的互操作
  - `global_asm!`插入
  - 内联汇编？？
- 怎样实现自己的print！宏
- console.rs没看懂
- 整理模块划分





成功运行！

![runwell](/home/lyf/Desktop/os-learning/rCore-labs/notes/imgs/lab0_notes_runwell.png)