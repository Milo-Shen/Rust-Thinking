#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

pub fn generics() {
    // 提前声明，跟泛型函数定义类似，首先我们在使用泛型参数之前必需要进行声明 Point<T>，接着就可以在结构体的字段类型中使用 T 来替代具体的类型
    // x 和 y 是相同的类型
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("{:?}, {:?}", integer, float);

    // 枚举中使用泛型
    // 提到枚举类型，Option 永远是第一个应该被想起来的，在之前的章节中，它也多次出现：
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}
