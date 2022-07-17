pub fn learn_vec() {
    // 先不运行，来推断下结果，首先 first = &v[0] 进行了不可变借用，v.push 进行了可变借用，如果 first 在 v.push 之后不再使用，那么该段代码可以成功编译（原因见引用的作用域）。
    // 原因在于：数组的大小是可变的，当旧数组的大小不够用时，Rust 会重新分配一块更大的内存空间，然后把旧数组拷贝过来。这种情况下，之前的引用显然会指向一块无效的内存，这非常 rusty —— 对用户进行严格的教育。
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {}", first);

    // 存储不同类型的元素
    // 在本节开头，有讲到数组的元素必需类型相同，但是也提到了解决方案：那就是通过使用枚举类型和特征对象来实现不同类型元素的存储。先来看看通过枚举如何实现：
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }

    fn show_addr(ip: IpAddr) {
        println!("{:?}", ip);
    }

    let v = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];

    for ip in v {
        show_addr(ip)
    }
}
