use std::fmt::{Debug, Display};
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

    // 上面代码有两点值得注意：
    // 1. &'static 的引用确实可以和程序活得一样久，因为我们通过 get_str_at_location 函数直接取到了对应的字符串
    // 2. 持有 &'static 引用的变量，它的生命周期受到作用域的限制，大家务必不要搞混了

    // T: 'static
    // 相比起来，这种形式的约束就有些复杂了。
    // 首先，在以下两种情况下，T: 'static 与 &'static 有相同的约束：T 必须活得和程序一样久。
    fn print_it<T: Debug + 'static>(input: T) {
        println!("'static value passed in is: {:?}", input);
    }

    fn print_it1(input: impl Debug + 'static) {
        println!("'static value passed in is: {:?}", input);
    }

    let i = 5;
    // print_it(&i);
    // print_it1(&i);
    // 以上代码会报错，原因很简单: &i 的生命周期无法满足 'static 的约束，如果大家将 i 修改为常量，那自然一切 OK。

    fn print_it2<T: Debug + 'static>(input: &T) {
        println!("'static value passed in is: {:?}", input);
    }
    print_it2(&i);
    // 这段代码竟然不报错了！原因在于我们约束的是 T，但是使用的却是它的引用 &T，换而言之，我们根本没有直接使用 T，因此编译器就没有去检查 T 的生命周期约束！它只要确保 &T 的生命周期符合规则即可，在上面代码中，它自然是符合的。

    fn static_bound<T: Display + 'static>(t: &T) {
        println!("{}", t);
    }
    let r1;
    let r2;
    {
        static STATIC_EXAMPLE: i32 = 42;
        r1 = &STATIC_EXAMPLE;
        let x = "&'static str";
        r2 = x;
        // r1 和 r2 持有的数据都是 'static 的，因此在花括号结束后，并不会被释放
    }

    println!("&'static i32: {}", r1); // -> 42
    println!("&'static str: {}", r2); // -> &'static str

    let r3: &str;

    {
        let s1 = "String".to_string();

        // s1 虽然没有 'static 生命周期，但是它依然可以满足 T: 'static 的约束
        // 充分说明这个约束是多么的弱。。
        static_bound(&s1);

        // s1 是 String 类型，没有 'static 的生命周期，因此下面代码会报错
        // r3 = &s1;

        // s1 在这里被 drop
    }
    // println!("{}", r3);

    // static 到底针对谁 ？
    // 大家有没有想过，到底是 &'static 这个引用还是该引用指向的数据活得跟程序一样久呢？
    // 答案是引用指向的数据，而引用本身是要遵循其作用域范围的，我们来简单验证下：
}
