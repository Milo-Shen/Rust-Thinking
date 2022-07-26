use std::fmt;
use std::ops::Add;

pub fn newtype_and_type_aliases() {
    // newtype
    // 何为 newtype？简单来说，就是使用元组结构体的方式将已有的类型包裹起来：struct Meters(u32);，那么此处 Meters 就是一个 newtype。
    // 为何需要 newtype？Rust 这多如繁星的 Old 类型满足不了我们吗？这是因为：
    // 1. 自定义类型可以让我们给出更有意义和可读性的类型名，例如与其使用 u32 作为距离的单位类型，我们可以使用 Meters，它的可读性要好得多
    // 2. 对于某些场景，只有 newtype 可以很好地解决
    // 3. 隐藏内部类型的细节

    // 为外部类型实现外部特征
    // 在之前的章节中，我们有讲过，如果在外部类型上实现外部特征必须使用 newtype 的方式，否则你就得遵循孤儿规则：要为类型 A 实现特征 T，那么 A 或者 T 必须至少有一个在当前的作用范围内。
    // 例如，如果想使用 println!("{}", v) 的方式去格式化输出一个动态数组 Vec，以期给用户提供更加清晰可读的内容，那么就需要为 Vec 实现 Display 特征，但是这里有一个问题： Vec 类型定义在标准库中，Display 亦然，这时就可以祭出大杀器 newtype 来解决：
    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
    // 如上所示，使用元组结构体语法 struct Wrapper(Vec<String>) 创建了一个 newtype Wrapper，然后为它实现 Display 特征，最终实现了对 Vec 动态数组的格式化输出。

    // 更好的可读性及类型异化
    // 首先，更好的可读性不等于更少的代码（如果你学过 Scala，相信会深有体会），其次下面的例子只是一个示例，未必能体现出更好的可读性：
    struct Meters(u32);
    impl fmt::Display for Meters {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "目标地点距离你{}米", self.0)
        }
    }

    impl Add for Meters {
        type Output = Self;

        fn add(self, other: Meters) -> Self {
            Self(self.0 + other.0)
        }
    }

    fn calculate_distance(d1: Meters, d2: Meters) -> Meters {
        d1 + d2
    }

    let d = calculate_distance(Meters(10), Meters(20));
    println!("{}", d);
    // 事实上，除了可读性外，还有一个极大的优点：如果给 calculate_distance 传一个其它的类型，例如 struct MilliMeters(u32);，该代码将无法编译。尽管 Meters 和 MilliMeters 都是对 u32 类型的简单包装，但是它们是不同的类型！

    // 隐藏内部类型的细节
    // 众所周知，Rust 的类型有很多自定义的方法，假如我们把某个类型传给了用户，但是又不想用户调用这些方法，就可以使用 newtype：
    struct MilliMeters(u32);
    let i: u32 = 2;
    assert_eq!(i.pow(2), 4);
    let n = MilliMeters(i);
    // 下面的代码将报错，因为`Meters`类型上没有`pow`方法
    // assert_eq!(n.pow(2), 4);
    // 不过需要偷偷告诉你的是，这种方式实际上是掩耳盗铃，因为用户依然可以通过 n.0.pow(2) 的方式来调用内部类型的方法 :)

    // 类型别名(Type Alias)
    // 除了使用 newtype，我们还可以使用一个更传统的方式来创建新类型：类型别名
    type MyMeters = u32;
    // 嗯，不得不说，类型别名的方式看起来比 newtype 顺眼的多，而且跟其它语言的使用方式几乎一致，但是： 类型别名并不是一个独立的全新的类型，而是某一个类型的别名，因此编译器依然会把 Meters 当 u32 来使用：
    let x: u32 = 5;
    let y: MyMeters = 5;
    println!("x + y = {}", x + y);
    // 上面的代码将顺利编译通过，但是如果你使用 newtype 模式，该代码将无情报错，简单做个总结：
    // 1. 类型别名仅仅是别名，只是为了让可读性更好，并不是全新的类型，newtype 才是！
    // 2. 类型别名无法实现为外部类型实现外部特征等功能，而 newtype 可以
}
