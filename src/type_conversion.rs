pub fn type_conversion() {
    // 每个类型能表达的数据范围不同，如果把范围较大的类型转换成较小的类型，会造成错误，因此我们需要把范围较小的类型转换成较大的类型，来避免这些问题的发生。
    let a = i8::MAX;
    println!("{}", a);

    let a = 3.1 as i8;
    let b = 100_i8 as i32;
    let c = 'a' as u8;
    println!("{},{},{}", a, b, c)
}
