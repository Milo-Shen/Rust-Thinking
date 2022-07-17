pub fn learn_vec() {
    // 先不运行，来推断下结果，首先 first = &v[0] 进行了不可变借用，v.push 进行了可变借用，如果 first 在 v.push 之后不再使用，那么该段代码可以成功编译（原因见引用的作用域）。
    // 原因在于：数组的大小是可变的，当旧数组的大小不够用时，Rust 会重新分配一块更大的内存空间，然后把旧数组拷贝过来。这种情况下，之前的引用显然会指向一块无效的内存，这非常 rusty —— 对用户进行严格的教育。
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {}", first);
}
