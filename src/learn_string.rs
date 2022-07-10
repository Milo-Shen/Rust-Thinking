pub fn learning_string() {
    // Rust 中的字符是 Unicode 类型，因此每个字符占据 4 个字节内存空间，
    // 但是在字符串中不一样，字符串是 UTF-8 编码，也就是字符串中的字符所占的字节数是变化的(1 - 4)
    // 当 Rust 用户提到字符串时，往往指的就是 String 类型和 &str 字符串切片类型，这两个类型都是 UTF-8 编码。
    let s = String::from("hello,world!");
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());

    // 如果问你该字符串多长，你可能会说 3，但是实际上是 9 个字节的长度，
    // 因为大部分常用汉字在 UTF-8 中的长度是 3 个字节，因此这种情况下对 hello 进行索引，访问 &hello[0] 没有任何意义，
    // 因为你取不到 中 这个字符，而是取到了这个字符三个字节中的第一个字节，这是一个非常奇怪而且难以理解的返回值。
    let hello = String::from("中国人");
    println!("{}", hello);

    let a = String::from("नमस्ते");
    let a_vec: Vec<char> = a.chars().collect();
    // a 是以字母序的方式去看待字符串, a_vec 是以字符 char 的形式去看待字符串, 结果是不同的
    println!("{} ,{:?}", a, a_vec);

    // 还有一个原因导致了 Rust 不允许去索引字符串：因为索引操作，我们总是期望它的性能表现是 O(1)，
    // 然而对于 String 类型来说，无法保证这一点，因为 Rust 可能需要从 0 开始去遍历字符串来定位合法的字符。
}

fn say_hello(s: &str) {
    println!("{}", s);
}
