#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // self、&self 和 &mut self
    // 接下里的内容非常重要，请大家仔细看。在 area 的签名中，我们使用 &self 替代 rectangle: &Rectangle，&self 其实是 self: &Self 的简写（注意大小写）。
    // 在一个 impl 块内，Self 指代被实现方法的结构体类型，self 指代此类型的实例，换句话说，self 指代的是 Rectangle 结构体实例，这样的写法会让我们的代码简洁很多.
    // 而且非常便于理解：我们为哪个结构体实现方法，那么 self 就是指代哪个结构体的实例。

    // 需要注意的是，self 依然有所有权的概念：
    // 1. self 表示 Rectangle 的所有权转移到该方法中，这种形式用的较少
    // 2. &self 表示该方法对 Rectangle 的不可变借用
    // 3. &mut self 表示可变借用
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 方法名跟结构体字段名相同
    fn width(&self) -> bool {
        self.width > 0
    }
}

pub fn method() {
    // 仅仅通过使用 self 作为第一个参数来使方法获取实例的所有权是很少见的
    // 这种使用方式往往用于把当前的对象转成另外一个对象时使用，转换完后，就不再关注之前的对象，且可以防止对之前对象的误调用。

    // 简单总结下，使用方法代替函数有以下好处：
    // 1. 不用在函数签名中重复书写 self 对应的类型
    // 2. 代码的组织性和内聚性更强，对于代码维护和阅读来说，好处巨大
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // 当我们使用 rect1.width() 时，Rust 知道我们调用的是它的方法，如果使用 rect1.width，则是访问它的字段。
    println!("The width of the rectangle is {}", rect1.width());

    // 一般来说，方法跟字段同名，往往适用于实现 getter 访问器.
    // 用这种方式，我们可以把 Rectangle 的字段设置为私有属性，只需把它的 new 和 width 方法设置为公开可见，那么用户就可以创建一个矩形，同时通过访问器 rect1.width() 方法来获取矩形的宽度，因为 width 字段是私有的，当用户访问 rect1.width 字段时，就会报错。注意在此例中，Self 指代的就是被实现方法的结构体 Rectangle。
    pub struct Rectangle1 {
        width: u32,
        height: u32,
    }

    impl Rectangle1 {
        pub fn new(width: u32, height: u32) -> Self {
            Rectangle1 { width, height }
        }
        pub fn width(&self) -> u32 {
            return self.width;
        }

        pub fn height(&self) -> u32 {
            return self.height;
        }
    }

    let rect1 = Rectangle1::new(10, 10);

    println!(
        "The width of the Rectangle1 is {} square pixels.",
        rect1.width()
    );

    println!(
        "The height of the Rectangle1 is {} square pixels.",
        rect1.height()
    );
}
