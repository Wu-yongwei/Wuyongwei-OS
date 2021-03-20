//! # 全局属性
//! - `#![no_std]` 
//! 禁⽤标准库
#![no_std]
//!
//! - `#![no_main]` 
//! 不使⽤ `main` 函数等全部 Rust-level ⼊⼝点来作为程序⼊⼝
#![no_main]
//! # ⼀些 unstable 的功能需要在 crate 层级声明后才可以使⽤
//! - `#![feature(llvm_asm)]` 
//! 内嵌汇编
#![feature(llvm_asm)]
//!
//! - `#![feature(global_asm)]` 
//! 内嵌整个汇编⽂件
#![feature(global_asm)]
//!
//! - `#![feature(panic_info_message)]` 
//! panic! 时，获取其中的信息并打印
#![feature(panic_info_message)]
#[macro_use]
mod console;
mod panic;
mod sbi;
mod interrupt;
// 汇编编写的程序⼊⼝，具体⻅该⽂件
global_asm!(include_str!("entry.asm"));
/// Rust 的⼊⼝函数
///
/// 在 `_start` 为我们进⾏了⼀系列准备之后，这是第⼀个被调⽤的 Rust 函数
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    println!("Hello 吴咏蔚!");
    // 初始化各种模块
    interrupt::init();
    
    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    };
    
    loop{};
    
    unreachable!();
    
    panic!("end of rust_main");
    
}
