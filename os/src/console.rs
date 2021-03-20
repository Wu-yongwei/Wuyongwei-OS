//! 实现控制台的字符输⼊和输出
//!
//! # 格式化输出
//!
//! ['core::fmt::Write'] trait 包含
//! - 需要实现的 ['write_str'] ⽅法
//! - 自带实现，但依赖于 ['write_str'] 的 ['write_fmt'] ⽅法
//!
//! 我们声明⼀个类型，为其实现 ['write_str'] ⽅法后，就可以使⽤ ['write_fmt'] 来进⾏格式化输出
//!
//! ['write_str']: core::fmt::Write::write_str
//! ['write_fmt']: core::fmt::Write::write_fmt
use crate::sbi::*;
use core::fmt::{self, Write};
/// ⼀个 [Zero-Sized Type]，实现 ['core::fmt::Write'] trait 来进行格式化输出
///
/// ZST 只可能有⼀个值（即为空），因此它本身就是⼀个单件
struct Stdout;
impl Write for Stdout {
 /// 打印⼀个字符串
 ///
 /// ['console_putchar'] sbi 调用每次接受⼀个 'usize'，但实际上会把它作为 'u8' 来打印字符。
 /// 因此，如果字符串中存在非ASCII 字符，需要在 utf-8 编码下，对于每⼀个 'u8' 调用⼀次 ['console_putchar']
 fn write_str(&mut self, s: &str) -> fmt::Result {
 	let mut buffer = [0u8; 4];
	for c in s.chars() {
 		for code_point in c.encode_utf8(&mut buffer).as_bytes().iter() {
 			console_putchar(*code_point as usize);
 		}
 	}
 	Ok(())
 }
}
/// 打印由 ['core::format_args!'] 格式化后的数据
///
/// ['print!'] 和 ['println!'] 宏都将展开成此函数
///
/// ['core::format_args!']: https://doc.rust-lang.org/nightly/core/macro.format_args.html
pub fn print(args: fmt::Arguments) {
 Stdout.write_fmt(args).unwrap();
}
/// 实现类似于标准库中的 'print!' 宏
///
/// 使⽤实现了 ['core::fmt::Write'] trait 的 ['console::Stdout']
#[macro_export]
macro_rules! print {
 ($fmt: literal $(, $($arg: tt)+)?) => {
 $crate::console::print(format_args!($fmt $(, $($arg)+)?));
 }
}
/// 实现类似于标准库中的 'println!' 宏
///
/// 使用实现了 ['core::fmt::Write'] trait 的 ['console::Stdout']
#[macro_export]
macro_rules! println {
 ($fmt: literal $(, $($arg: tt)+)?) => {
 $crate::console::print(format_args!(concat!($fmt, "\n") $(,
$($arg)+)?));
 }
}
