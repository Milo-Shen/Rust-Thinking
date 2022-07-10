pub fn tuple() {
    let tup = (500, 6.4, 1);

    // 用模式匹配解构元组
    let (x, y, z) = tup;
    println!("The value of x, y, z is: {}, {}, {}", x, y, z);

    // 用 . 来访问元组
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("{}, {}, {}", five_hundred, six_point_four, one);

    // 元组的使用示例
    // 元组在函数返回值场景很常用，例如下面的代码，可以使用元组返回多个值
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度
    (s, length)
}
