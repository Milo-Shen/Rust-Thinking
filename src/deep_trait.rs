use std::{fmt, ops::Add};

pub fn deep_trait() {
    // 关联类型
    // 关联类型是在特征定义的语句块中，申明一个自定义类型，这样就可以在特征的方法签名中使用该类型：
    pub trait Iterator {
        // Item 就是关联类型，用于替代遍历的值的类型。
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }

    //  Self 用来指代当前调用者的具体类型，那么 Self::Item 就用来指代该类型实现中定义的 Item 类型：
    #[derive(Debug)]
    struct Counter {}

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            Some(0)
        }
    }

    let mut c = Counter {};
    println!("c next: {:?}", c.next());

    // 为了代码的可读性，当你使用了泛型后，你需要在所有地方都写 Iterator<Item>，而使用了关联类型，你只需要写 Iterator，当类型定义复杂时，这种写法可以极大的增加可读性：
    pub trait CacheableItem: Clone + Default + fmt::Debug {
        type Address: AsRef<[u8]> + Clone + fmt::Debug + Eq;
        fn is_null(&self) -> bool;
    }

    // 例如上面的代码，Address 的写法自然远比 AsRef<[u8]> + Clone + fmt::Debug + Eq + Hash 要简单的多，而且含义清晰。
    // 再例如，如果使用泛型，你将得到以下的代码：
    trait Container<A, B> {
        fn contains(&self, a: A, b: B) -> bool;
    }

    fn difference<A, B, C>(container: &C) -> i32
    where
        C: Container<A, B>,
    {
        1
    }

    // 可以看到，由于使用了泛型，导致函数头部也必须增加泛型的声明，而使用关联类型，将得到可读性好得多的代码：
    trait Container1 {
        type A;
        type B;
        fn contains(&self, a: Self::A, b: Self::B) -> bool;
    }

    fn difference1<C: Container1>(container: &C) {}

    trait Container2 {
        type A;
        type B;
        fn contains(&self, a: &Self::A, b: &Self::B);
    }

    struct ContainerExp<A, B> {
        a: A,
        b: B,
    }

    impl<A, B> Container2 for ContainerExp<A, B> {
        type A = u32;
        type B = u32;
        fn contains(&self, a: &Self::A, b: &Self::B) {
            println!("A: {}, B: {}", a, b);
        }
    }

    // todo: add example for this
    let container_exp_1 = ContainerExp { a: 1, b: 1 };
    container_exp_1.contains(&container_exp_1.a, &container_exp_1.b);

    // 默认泛型类型参数
    // 当使用泛型类型参数时，可以为其指定一个默认的具体类型，例如标准库中的 std::ops::Add 特征：
    // trait Add<RHS = Self> {
    //     type Output;

    //     fn add(self, rhs: RHS) -> Self::Output;
    // }
    // 它有一个泛型参数 RHS，但是与我们以往的用法不同，这里它给 RHS 一个默认值，也就是当用户不指定 RHS 时，默认使用两个同样类型的值进行相加，然后返回一个关联类型 Output。

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // 上面的代码主要干了一件事，就是为 Point 结构体提供 + 的能力，这就是运算符重载，不过 Rust 并不支持创建自定义运算符，你也无法为所有运算符进行重载，目前来说，只有定义在 std::ops 中的运算符才能进行重载。
    // 跟 + 对应的特征是 std::ops::Add，我们在之前也看过它的定义 trait Add<RHS=Self>，但是上面的例子中并没有为 Point 实现 Add<RHS> 特征，而是实现了 Add 特征（没有默认泛型类型参数），这意味着我们使用了 RHS 的默认类型，也就是 Self。换句话说，我们这里定义的是两个相同的 Point 类型相加，因此无需指定 RHS。

    // 与上面的例子相反，下面的例子，我们来创建两个不同类型的相加：
    // 这里，是进行 Millimeters + Meters 两种数据类型的 + 操作，因此此时不能再使用默认的 RHS，否则就会变成 Millimeters + Millimeters 的形式。
    // 使用 Add<Meters> 可以将 RHS 指定为 Meters，那么 fn add(self, rhs: RHS) 自然而言的变成了 Millimeters 和 Meters 的相加。

    // 默认类型参数主要用于两个方面：
    // 1. 减少实现的样板代码
    // 2. 扩展类型但是无需大幅修改现有的代码
    #[derive(Debug)]
    struct Millimeters(u32);
    #[derive(Debug)]
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Self::Output {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    let millimeter = Millimeters(10);
    let meter = Meters(5);
    // 因为是为 Millimeters 实现的 Add 操作, 所以做加法时候 Millimeters
    let final_millimeter = millimeter + meter;
    println!("{:?}", final_millimeter);

    // 调用同名的方法
    // 不同特征拥有同名的方法是很正常的事情，你没有任何办法阻止这一点；甚至除了特征上的同名方法外，在你的类型上，也有同名方法：
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    // 优先调用类型上的方法
    // 这段代码会打印 *waving arms furiously*，说明直接调用了类型上定义的方法。
    let person = Human;
    person.fly();

    // 调用特征上的方法
    // 为了能够调用两个特征的方法，需要使用显式调用的语法：
    let person = Human;
    Pilot::fly(&person); // 调用Pilot特征上的方法
    Wizard::fly(&person); // 调用Wizard特征上的方法
    person.fly(); // 调用Human类型自身的方法
}
