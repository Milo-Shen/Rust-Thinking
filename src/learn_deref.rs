use std::ops::Deref;

pub fn learn_deref() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    impl Person {
        fn new(name: String, age: u8) -> Self {
            Person { name, age }
        }

        fn display(self: &mut Person, age: u8) {
            let Person { name, age } = &self;
        }
    }

    // 以上代码有一个很奇怪的地方：在 display 方法中，self 是 &mut Person 的类型，接着我们对其取了一次引用 &self，此时 &self 的类型是 &&mut Person，然后我们又将其和 Person 类型进行匹配，取出其中的值。

    // 何为智能指针？能不让你写出 ****s 形式的解引用，我认为就是智能: )，智能指针的名称来源，主要就在于它实现了 Deref 和 Drop 特征，这两个特征可以智能地帮助我们节省使用上的负担：
    // 1. Deref 可以让智能指针像引用那样工作，这样你就可以写出同时支持智能指针和引用的代码，例如 *T
    // 2. Drop 允许你指定智能指针超出作用域后自动执行的代码，例如做一些数据清除等收尾工作

    // 先来看看 Deref 特征是如何工作的。

    // 通过 * 获取引用背后的值
    // 在正式讲解 Deref 之前，我们先来看下常规引用的解引用。
    // 常规引用是一个指针类型，包含了目标数据存储的内存地址。对常规引用使用 * 操作符，就可以通过解引用的方式获取到内存地址对应的数据值：
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    // 这里 y 就是一个常规引用，包含了值 5 所在的内存地址，然后通过解引用 *y，我们获取到了值 5。如果你试图执行 assert_eq!(5, y);，代码就会无情报错，因为你无法将一个引用与一个数值做比较

    // 智能指针解引用
    // 上面所说的解引用方式和其它大多数语言并无区别，但是 Rust 中将解引用提升到了一个新高度。考虑一下智能指针，它是一个结构体类型，如果你直接对它进行 *myStruct，显然编译器不知道该如何办，因此我们可以为智能指针结构体实现 Deref 特征。
    // 实现 Deref 后的智能指针结构体，就可以像普通引用一样，通过 * 进行解引用，例如 Box<T> 智能指针：
    let x = Box::new(1);
    let sum = *x + 1;
    println!("sum = {sum}");
    // 智能指针 x 被 * 解引用为 i32 类型的值 1，然后再进行求和。

    // 定义自己的智能指针
    // 现在，让我们一起来实现一个智能指针，功能上类似 Box<T>。由于 Box<T> 本身很简单，并没有包含类如长度、最大长度等信息，因此用一个元组结构体即可。
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    // 跟 Box<T> 一样，我们的智能指针也持有一个 T 类型的值，然后使用关联函数 MyBox::new 来创建智能指针。由于还未实现 Deref 特征，此时使用 * 肯定会报错
    // 为智能指针实现 Deref 特征

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    // 很简单，当解引用 MyBox 智能指针时，返回元组结构体中的元素 &self.0，有几点要注意的：
    // 1. 在 Deref 特征中声明了关联类型 Target，在之前章节中介绍过，关联类型主要是为了提升代码可读性
    // 2. deref 返回的是一个常规引用，可以被 * 进行解引用

    // 之前报错的代码此时已能顺利编译通过。当然，标准库实现的智能指针要考虑很多边边角角情况，肯定比我们的实现要复杂。
    let y = MyBox::new(5);
    println!("y = {}", *y);

    // * 背后的原理
    // 当我们对智能指针 Box 进行解引用时，实际上 Rust 为我们调用了以下方法：
    let c = *(y.deref());
    println!("c = {c}");

    // 首先调用 deref 方法返回值的常规引用，然后通过 * 对常规引用进行解引用，最终获取到目标值。
    // 至于 Rust 为何要使用这个有点啰嗦的方式实现，原因在于所有权系统的存在。如果 deref 方法直接返回一个值，而不是引用，那么该值的所有权将被转移给调用者，而我们不希望调用者仅仅只是 *T 一下，就拿走了智能指针中包含的值。
    // 需要注意的是，* 不会无限递归替换，从 *y 到 *(y.deref()) 只会发生一次，而不会继续进行替换然后产生形如 *((y.deref()).deref()) 的怪物。

    let s = MyBox::new(String::from("hello world"));
    // move occurs because value has type `String`, which does not implement the `Copy` trait
    // let mys = *s;

    // 函数和方法中的隐式 Deref 转换
    // 对于函数和方法的传参，Rust 提供了一个极其有用的隐式转换：Deref 转换。若一个类型实现了 Deref 特征，那它的引用在传给函数或方法时，会根据参数签名来决定是否进行隐式的 Deref 转换，例如：
    fn display(s: &str) {
        println!("{}", s);
    }
    let s = String::from("hello world");
    display(&s);

    // 以上代码有几点值得注意：
    // 1. String 实现了 Deref 特征，可以在需要时自动被转换为 &str 类型
    // 2. &s 是一个 &String 类型，当它被传给 display 函数时，自动通过 Deref 转换成了 &str
    // 3. 必须使用 &s 的方式来触发 Deref(仅引用类型的实参才会触发自动解引用)

    // 连续的隐式 Deref 转换
    // 如果你以为 Deref 仅仅这点作用，那就大错特错了。Deref 可以支持连续的隐式转换，直到找到适合的形式为止：
    let s = MyBox::new(String::from("hello world"));
    display(&s);
    // 这里我们使用了之前自定义的智能指针 MyBox，并将其通过连续的隐式转换变成 &str 类型：首先 MyBox 被 Deref 成 String 类型，结果并不能满足 display 函数参数的要求，编译器发现 String 还可以继续 Deref 成 &str，最终成功的匹配了函数参数。
    // 想象一下，假如 Rust 没有提供这种隐式转换，我们该如何调用 display 函数？
    let m = MyBox::new(String::from("Rust"));
    display(&(*m)[..]);
    // 结果不言而喻，肯定是 &s 的方式优秀得多。总之，当参与其中的类型定义了 Deref 特征时，Rust 会分析该类型并且连续使用 Deref 直到最终获得一个引用来匹配函数或者方法的参数类型，这种行为完全不会造成任何的性能损耗，因为完全是在编译期完成。
    // 但是 Deref 并不是没有缺点，缺点就是：如果你不知道某个类型是否实现了 Deref 特征，那么在看到某段代码时，并不能在第一时间反应过来该代码发生了隐式的 Deref 转换。事实上，不仅仅是 Deref，在 Rust 中还有各种 From/Into 等等会给阅读代码带来一定负担的特征。还是那句话，一切选择都是权衡，有得必有失，得了代码的简洁性，往往就失去了可读性，Go 语言就是一个刚好相反的例子。
    let s = MyBox::new(String::from("hello, world"));
    let s1: &str = &s;
    let s2: String = s.to_string();
    // 对于 s1，我们通过两次 Deref 将 &str 类型的值赋给了它（赋值操作需要手动解引用）；
    // 而对于 s2，我们在其上直接调用方法 to_string，实际上 MyBox 根本没有没有实现该方法，能调用 to_string，完全是因为编译器对 MyBox 应用了 Deref 的结果（方法调用会自动解引用）。

    // Deref 规则总结
    // 在上面，我们零碎的介绍了不少关于 Deref 特征的知识，下面来通过较为正式的方式来对其规则进行下总结。
    // 一个类型为 T 的对象 foo，如果 T: Deref<Target=U>，那么，相关 foo 的引用 &foo 在应用的时候会自动转换为 &U。
    // 粗看这条规则，貌似有点类似于 AsRef，而跟 解引用 似乎风马牛不相及，实际里面有些玄妙之处。
}
