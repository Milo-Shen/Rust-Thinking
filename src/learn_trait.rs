use std::fmt::{Debug, Display};

pub fn learn_trait() {
    // 特征定义了一个可以被共享的行为，只要实现了特征，你就能使用该行为。

    // 定义特征
    // 如果不同的类型具有相同的行为，那么我们就可以定义一个特征，然后为这些类型实现该特征。定义特征是把一些方法组合在一起，目的是定义一个实现某些目标所必需的行为的集合。

    // 特征只定义行为看起来是什么样的，而不定义行为具体是怎么样的。因此，我们只定义特征方法的签名，而不进行实现，此时方法签名结尾是 ;，而不是一个 {}。
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    // 为类型实现特征
    pub struct Post {
        pub title: String,   // 标题
        pub author: String,  // 作者
        pub content: String, // 内容
    }

    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("文章{}, 作者是{}", self.title, self.author)
        }
    }

    #[derive(Debug)]
    pub struct Weibo {
        pub username: String,
        pub content: String,
    }

    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{}发表了微博{}", self.username, self.content)
        }
    }

    let post = Post {
        title: "Rust语言简介".to_string(),
        author: "Sunface".to_string(),
        content: "Rust棒极了!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "好像微博没Tweet好用".to_string(),
    };

    println!("{}", post.summarize());
    println!("{}", weibo.summarize());

    // 特征定义与实现的位置(孤儿规则)
    // 关于特征实现与定义的位置，有一条非常重要的原则：如果你想要为类型 A 实现特征 T，那么 A 或者 T 至少有一个是在当前作用域中定义的！。例如我们可以为上面的 Post 类型实现标准库中的 Display 特征，这是因为 Post 类型定义在当前的作用域中。同时，我们也可以在当前包中为 String 类型实现 Summary 特征，因为 Summary 定义在当前作用域中。
    // 但是你无法在当前作用域中，为 String 类型实现 Display 特征，因为它们俩都定义在标准库中，其定义所在的位置都不在当前作用域，跟你半毛钱关系都没有，看看就行了。
    // 该规则被称为孤儿规则，可以确保其它人编写的代码不会破坏你的代码，也确保了你不会莫名其妙就破坏了风马牛不相及的代码。

    // 默认实现
    // 你可以在特征中定义具有默认实现的方法，这样其它类型无需再实现该方法，或者也可以选择重载该方法：
    pub trait Summary1 {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    struct QQ {}
    impl Summary1 for QQ {}

    let qq = QQ {};
    println!("{}", qq.summarize());

    // 默认实现允许调用相同特征中的其他方法，哪怕这些方法没有默认实现。如此，特征可以提供很多有用的功能而只需要实现指定的一小部分内容。例如，我们可以定义 Summary 特征，使其具有一个需要实现的 summarize_author 方法，然后定义一个 summarize 方法，此方法的默认实现调用 summarize_author 方法：
    pub trait Summary2 {
        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    struct QQ1 {}
    impl Summary2 for QQ1 {
        fn summarize_author(&self) -> String {
            String::from("summarize_author")
        }
    }
    let qq = QQ1 {};
    println!("{}", qq.summarize());

    // 使用特征作为函数参数
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    notify(&post);

    // 特征约束(trait bound)
    pub fn notify1(item1: &impl Summary, item2: &impl Summary) {}
    // 如果函数两个参数是不同的类型，那么上面的方法很好，只要这两个类型都实现了 Summary 特征即可。但是如果我们想要强制函数的两个参数是同一类型呢？上面的语法就无法做到这种限制，此时我们只能使特征约束来实现：
    pub fn notify2<T: Summary>(item1: &T, item2: &T) {}
    // 泛型类型 T 说明了 item1 和 item2 必须拥有同样的类型，同时 T: Summary 说明了 T 必须实现 Summary 特征。

    // 多重约束
    // 除了单个约束条件，我们还可以指定多个约束条件，例如除了让参数实现 Summary 特征外，还可以让参数实现 Display 特征以控制它的格式化输出：
    pub fn notify3(item: &(impl Summary + Display)) {}
    pub fn notify4<T: Summary + Display>(item: &T) {}

    // Where 约束
    // 当特征约束变得很多时，函数的签名将变得很复杂：
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {}
    // 严格来说，上面的例子还是不够复杂，但是我们还是能对其做一些形式上的改进，通过 where：
    fn some_function1<T, U>(t: &T, u: &U)
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
    }

    // 使用特征约束有条件地实现方法或特征
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    // 也可以有条件地实现特征, 例如，标准库为任何实现了 Display 特征的类型实现了 ToString 特征：
    // impl<T: Display> ToString for T {
    //     // --snip--
    // }

    // 函数返回中的 impl Trait
    fn returns_summarizable() -> impl Summary + Debug {
        Weibo {
            username: String::from("sunface"),
            content: String::from("m1 max太厉害了，电脑再也不会卡"),
        }
    }

    println!("{:?}", returns_summarizable());

    // 但是这种返回值方式有一个很大的限制：只能有一个具体的类型，例如：
    // 报错提示我们 if 和 else 返回了不同的类型。如果想要实现返回不同的类型，需要使用下一章节中的特征对象。
    // fn returns_summarizable1(switch: bool) -> impl Summary {
    //     if switch {
    //         Post {
    //             title: String::from("Penguins win the Stanley Cup Championship!"),
    //             author: String::from("Iceburgh"),
    //             content: String::from(
    //                 "The Pittsburgh Penguins once again are the best \
    //                  hockey team in the NHL.",
    //             ),
    //         }
    //     } else {
    //         Weibo {
    //             username: String::from("horse_ebooks"),
    //             content: String::from("of course, as you probably already know, people"),
    //         }
    //     }
    // }

    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // 通过 derive 派生特征
    // 在本书中，形如 #[derive(Debug)] 的代码已经出现了很多次，这种是一种特征派生语法，被 derive 标记的对象会自动实现对应的默认特征代码，继承相应的功能。

    // 如果你要使用一个特征的方法，那么你需要引入该特征到当前的作用域中，我们在上面用到了 try_into 方法，因此需要引入对应的特征
    // 但是 Rust 又提供了一个非常便利的办法，即把最常用的标准库中的特征通过 std::prelude 模块提前引入到当前作用域中，其中包括了 std::convert::TryInto，你可以尝试删除第一行的代码 use ...，看看是否会报错。
    let a: i32 = 10;
    let b: u16 = 100;

    let b_ = b.try_into().unwrap();

    if a < b_ {
        println!("Ten is less than one hundred.");
    }
}
