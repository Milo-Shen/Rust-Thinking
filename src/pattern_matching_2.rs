pub fn pattern_matching_2() {
    // while let 条件循环
    // 一个与 if let 类似的结构是 while let 条件循环，它允许只要模式匹配就一直进行 while 循环。下面展示了一个使用 while let 的例子

    // Vec是动态数组
    let mut stack = Vec::new();

    // 向数组尾部插入元素
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // stack.pop从数组尾部弹出元素
    // 这个例子会打印出 3、2 接着是 1。pop 方法取出动态数组的最后一个元素并返回 Some(value)，如果动态数组是空的，将返回 None
    // 对于 while 来说，只要 pop 返回 Some 就会一直不停的循环。一旦其返回 None，while 循环停止。我们可以使用 while let 来弹出栈中的每一个元素。
    // 你也可以用 loop + if let 或者 match 来实现这个功能，但是会更加啰嗦。
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for 循环
    // 这里使用 enumerate 方法产生一个迭代器，该迭代器每次迭代会返回一个 (索引，值) 形式的元组，然后用 (index,value) 来匹配。
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // let 语句 - 该语句我们已经用了无数次了，它也是一种模式匹配
    let x = 5;
    println!("{}", x);

    // 这其中，x 也是一种模式绑定，代表将匹配的值绑定到变量 x 上。因此，在 Rust 中,变量名也是一种模式，只不过它比较朴素很不起眼罢了。
    // 下面将一个元组与模式进行匹配(模式和值的类型必需相同！)，然后把 1, 2, 3 分别绑定到 x, y, z 上。
    // 模式匹配要求两边的类型必须相同，否则就会导致报错
    // 对于元组来说，元素个数也是类型的一部分 ！
    let (x, y, z) = (1, 2, 3);
    println!("{}, {}, {}", x, y, z);

    // 函数参数
    // 函数参数也是模式: 其中 x 就是一个模式，你还可以在参数中匹配元组:
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }
    let point = (3, 5);
    print_coordinates(&point);

    // 类似 let 和 for、match 都必须要求完全覆盖匹配，才能通过编译( 不可驳模式匹配 )。

    // 但是对于 if let，就可以这样使用:
    // 因为 if let 允许匹配一种模式，而忽略其余的模式( 可驳模式匹配 )
    if let Some(x) = Some(2) {
        println!("{}", x);
    }
}
