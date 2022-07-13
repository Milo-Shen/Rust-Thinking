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
    // Option<T> 是一个拥有泛型 T 的枚举类型，它第一个成员是 Some(T)，存放了一个类型为 T 的值。得益于泛型的引入，我们可以在任何一个需要返回值的函数中，去使用 Option<T> 枚举类型来做为返回值，用于返回一个任意类型的值 Some(T)，或者没有值 None。
    enum Option<T> {
        Some(T),
        None,
    }

    // 如果函数正常运行，则最后返回一个 Ok(T)，T 是函数具体的返回值类型，如果函数异常运行，则返回一个 Err(E)，E 是错误类型。例如打开一个文件：如果成功打开文件，则返回 Ok(std::fs::File)，因此 T 对应的是 std::fs::File 类型；而当打开文件时出现问题时，返回 Err(std::io::Error)，E 对应的就是 std::io::Error 类型。
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}
