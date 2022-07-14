#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// 需要注意的是，这里的 Point<T> 不再是泛型声明，而是一个完整的结构体类型，因为我们定义的结构体就是 Point<T> 而不再是 Point。
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
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

    // 方法中使用泛型
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    // 除了结构体中的泛型参数，我们还能在该结构体的方法中定义额外的泛型参数，就跟泛型函数一样
    // 这个例子中，T,U 是定义在结构体 Point 上的泛型参数，V,W 是单独定义在方法 mixup 上的泛型参数，它们并不冲突，说白了，你可以理解为，一个是结构体泛型，一个是函数泛型。
    struct Point1<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point1<T, U> {
        fn mixup<V, W>(self, other: Point1<V, W>) -> Point1<T, W> {
            Point1 {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point1 { x: 5, y: 10.4 };
    let p2 = Point1 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // 为具体的泛型类型实现方法
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    // 这段代码意味着 Point<f32> 类型会有一个方法 distance_from_origin，而其他 T 不是 f32 类型的 Point<T> 实例则没有定义此方法。这个方法计算点实例与坐标(0.0, 0.0) 之间的距离，并使用了只能用于浮点型的数学运算符。
    // 这样我们就能针对特定的泛型类型实现某个特定的方法，对于其它泛型类型则没有定义该方法。
}
