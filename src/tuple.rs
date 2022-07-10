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
}
