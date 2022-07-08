struct Struct {
    e: i32,
}

pub fn destructuring_assignment() {
    let (a, b, c, d, e);

    // 这种使用方式跟之前的 let 保持了一致性，但是 let 会重新绑定，而这里仅仅是对之前绑定的变量进行再赋值。
    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有是一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    println!("{},{},{},{},{}", a, b, c, d, e);
}
