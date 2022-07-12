pub fn process_control() {
    let mut container = [5; 5];

    // 如果不使用引用的话，所有权会被转移（move）到 for 语句块中，后面就无法再使用这个集合了
    // 对于实现了 copy 特征的数组(例如 [i32; 10] )而言， for item in arr 并不会把 arr 的所有权转移，而是直接对其进行了拷贝，因此循环之后仍然可以使用 arr
    for item in &container {
        println!("{}", item)
    }

    for item in &mut container {
        *item = 1;
    }

    // 如果想在循环中，修改该元素，可以使用 mut 关键字：
    for item in &container {
        println!("{}", item)
    }

    // for item in collection  ->       for item in IntoIterator::into_iter(collection)	-> 转移所有权
    // for item in &collection ->	    for item in collection.iter()	                -> 不可变借用
    // for item in &mut collection ->	for item in collection.iter_mut()	            -> 可变借用

    // 如果想在循环中获取元素的索引:
    let a = [4, 3, 2, 1];
    // `.iter()` 方法把 `a` 数组变成一个迭代器
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }

    // 有同学可能会想到，如果我们想用 for 循环控制某个过程执行 10 次，但是又不想单独声明一个变量来控制这个流程，该怎么写 ？
    // 可以用 _ 来替代 i 用于 for 循环中，在 Rust 中 _ 的含义是忽略该值或者类型的意思，如果不使用 _，那么编译器会给你一个 变量未使用的 的警告。
    // for _ in 0..10 { }

    // 两种循环方式优劣对比
    // 以下代码，使用了两种循环方式 :
    // 第一种
    let collection = [1, 2, 3, 4, 5];
    for i in 0..collection.len() {
        let item = collection[i];
        println!("{}", item);
    }

    // 第二种
    for item in collection {
        println!("{}", item);
    }
}
