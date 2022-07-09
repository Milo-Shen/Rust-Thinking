pub fn char_bool_unit() {
    // 是的，在 Rust 语言中这些都是字符，Rust 的字符不仅仅是 ASCII，所有的 Unicode 值都可以作为 Rust 字符，包括单个的中文、日文、韩文、emoji 表情符号等等，都是合法的字符类型。Unicode 值的范围从 U+0000 ~ U+D7FF 和 U+E000 ~ U+10FFFF。不过“字符”并不是 Unicode 中的一个概念，所以人在直觉上对“字符”的理解和 Rust 的字符概念并不一致。
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';
    println!("{}, {}, {}, {}", c, z, g, heart_eyed_cat);

    let x = '中';
    println!(
        "字符'中'占用了 {} 字节的内存大小",
        std::mem::size_of_val(&x)
    );

    // 注意，我们还没开始讲字符串，但是这里提前说一下，和一些语言不同，Rust 的字符只能用 '' 来表示， "" 是留给字符串的
}
