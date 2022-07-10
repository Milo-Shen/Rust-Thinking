pub fn statements_expressions() {
    // 调用一个函数是表达式，因为会返回一个值，调用宏也是表达式，
    // 用花括号包裹最终返回一个值的语句块也是表达式，总之，能返回值，它就是表达式:
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    fn ret_unit_type() {
        let x = 1;
        // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
        if x > 1 {}
    }
    assert_eq!(ret_unit_type(), ());
}
