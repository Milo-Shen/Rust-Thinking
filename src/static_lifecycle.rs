use std::fmt::Display;
use std::{slice::from_raw_parts, str::from_utf8_unchecked};

pub fn static_lifecycle() {
    // &'static 和 T: 'static\
    // 'static 在 Rust 中是相当常见的，例如字符串字面值就具有 'static 生命周期:
    fn print_author(author: &'static str) {
        println!("{}", author);
    }
    let mark_twain: &str = "Samuel Clemens";
    print_author(mark_twain);

    // 除此之外，特征对象的生命周期也是 'static，例如这里所提到的。
    // 除了 &'static 的用法外，我们在另外一种场景中也可以见到 'static 的使用:
    fn print<T: Display + 'static>(message: &T) {
        println!("{}", message);
    }
    print(&String::from("Samuel Clemens"));

    // &'static
    // &'static 对于生命周期有着非常强的要求：一个引用必须要活得跟剩下的程序一样久，才能被标注为 &'static。
    // 对于字符串字面量来说，它直接被打包到二进制文件中，永远不会被 drop，因此它能跟程序活得一样久，自然它的生命周期是 'static。
    // 但是，&'static 生命周期针对的仅仅是引用，而不是持有该引用的变量，对于变量来说，还是要遵循相应的作用域规则 :

    fn get_memory_location() -> (usize, usize) {
        // “Hello World” 是字符串字面量，因此它的生命周期是 `'static`.
        // 但持有它的变量 `string` 的生命周期就不一样了，它完全取决于变量作用域，对于该例子来说，也就是当前的函数范围
        let string = "Hello World!";
        let pointer = string.as_ptr() as usize;
        let length = string.len();
        (pointer, length)
        // `string` 在这里被 drop 释放
        // 虽然变量被释放，无法再被访问，但是数据依然还会继续存活
    }

    fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
        // 使用裸指针需要 `unsafe{}` 语句块
        unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
    }

    let (pointer, length) = get_memory_location();
    let message = get_str_at_location(pointer, length);
    println!(
        "The {} bytes at 0x{:X} stored: {}",
        length, pointer, message
    );
    // 如果大家想知道为何处理裸指针需要 `unsafe`，可以试着反注释以下代码
    // let message = get_str_at_location(1000, 10);
}
