pub fn trait_obj() {
    // impl Trait 的返回值类型并不支持多种不同的类型返回
    // 特征对象定义
    // 特征对象指向实现了 Draw 特征的类型的实例，也就是指向了 Button 或者 SelectBox 的实例，这种映射关系是存储在一张表中，可以在运行时通过特征对象找到具体调用的类型方法。

    // 可以通过 & 引用或者 Box<T> 智能指针的方式来创建特征对象。
    trait Draw {
        fn draw(&self) -> String;
    }

    impl Draw for u8 {
        fn draw(&self) -> String {
            format!("u8: {}", *self)
        }
    }

    impl Draw for f64 {
        fn draw(&self) -> String {
            format!("f64: {}", *self)
        }
    }

    // 若 T 实现了 Draw 特征， 则调用该函数时传入的 Box<T> 可以被隐式转换成函数参数签名中的 Box<dyn Draw>
    fn draw1(x: Box<dyn Draw>) {
        // 由于实现了 Deref 特征，Box 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法
        x.draw();
    }

    fn draw2(x: &dyn Draw) {
        x.draw();
    }

    let x = 1.1f64;
    // do_something(&x);
    let y = 8u8;

    // x 和 y 的类型 T 都实现了 `Draw` 特征，因为 Box<T> 可以在函数调用时隐式地被转换为特征对象 Box<dyn Draw>
    // 基于 x 的值创建一个 Box<f64> 类型的智能指针，指针指向的数据被放置在了堆上
    draw1(Box::new(x));
    // 基于 y 的值创建一个 Box<u8> 类型的智能指针
    draw1(Box::new(y));
    draw2(&x);
    draw2(&y);

    // draw1 函数的参数是 Box<dyn Draw> 形式的特征对象，该特征对象是通过 Box::new(x) 的方式创建的
    // draw2 函数的参数是 &dyn Draw 形式的特征对象，该特征对象是通过 &x 的方式创建的
    // dyn 关键字只用在特征对象的类型声明上，在创建时无需使用 dyn
    // 使用泛型和 特征约束，因为实现更清晰，且性能更好(特征对象，需要在运行时从 vtable 动态查找需要调用的方法)。

    pub trait Draw1 {
        fn draw(&self);
    }

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw1 for Button {
        fn draw(&self) {
            println!(
                "button draw 1, weight: {}, height: {}, label: {}",
                self.width, self.height, self.label
            );
        }
    }

    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw1 for SelectBox {
        fn draw(&self) {
            println!(
                "SelectBox draw 1, weight: {}, height: {}, options: {:?}",
                self.width, self.height, self.options
            );
        }
    }

    pub struct Screen {
        pub components: Vec<Box<dyn Draw1>>,
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    // 注意 dyn 不能单独作为特征对象的定义，例如下面的代码编译器会报错，原因是特征对象可以是任意实现了某个特征的类型，编译器在编译期不知道该类型的大小，不同的类型大小是不同的。
    // 而 &dyn 和 Box<dyn> 在编译期都是已知大小，所以可以用作特征对象的定义。
    // let screen = Screen {
    //     components: vec![Box::new(String::from("Hi"))],
    // };

    // 特征对象的动态分发
    // 回忆一下泛型章节我们提到过的，泛型是在编译期完成处理的：编译器会为每一个泛型参数对应的具体类型生成一份代码，这种方式是静态分发(static dispatch)，因为是在编译期完成的，对于运行期性能完全没有任何影响。
    // 与静态分发相对应的是动态分发(dynamic dispatch)，在这种情况下，直到运行时，才能确定需要调用什么方法。之前代码中的关键字 dyn 正是在强调这一“动态”的特点。

    // 当使用特征对象时，Rust 必须使用动态分发。编译器无法知晓所有可能用于特征对象代码的类型，所以它也不知道应该调用哪个类型的哪个方法实现。
    // 为此，Rust 在运行时使用特征对象中的指针来知晓需要调用哪个方法。动态分发也阻止编译器有选择的内联方法代码，这会相应的禁用一些优化。

    // 结合上文的内容和这张图可以了解：
    // 1. 特征对象大小不固定
    // 2. 几乎总是使用特征对象的引用方式
    //    1. 虽然特征对象没有固定大小，但它的引用类型的大小是固定的，它由两个指针组成（ptr 和 vptr），因此占用两个指针大小
    //    2. 一个指针 ptr 指向实现了特征 Draw 的具体类型的实例，也就是当作特征 Draw 来用的类型的实例，比如类型 Button 的实例、类型 SelectBox 的实例
    //    3. 另一个指针 vptr 指向一个虚表 vtable，vtable 中保存了类型 Button 或类型 SelectBox 的实例对于可以调用的实现于特征 Draw 的方法。当调用方法时，直接从 vtable 中找到方法并调用。之所以要使用一个 vtable 来保存各实例的方法，是因为实现了特征 Draw 的类型有多种，这些类型拥有的方法各不相同，当将这些类型的实例都当作特征 Draw 来使用时(此时，它们全都看作是特征 Draw 类型的实例)，有必要区分这些实例各自有哪些方法可调用
    // 一定要注意，此时的 btn 是 Draw 的特征对象的实例，而不再是具体类型 Button 的实例，而且 btn 的 vtable 只包含了实现自特征 Draw 的那些方法（比如 draw），因此 btn 只能调用实现于特征 Draw 的 draw 方法，而不能调用类型 Button 本身实现的方法和类型 Button 实现于其他特征的方法。也就是说，btn 是哪个特征对象的实例，它的 vtable 中就包含了该特征的方法。

    // Self 与 self
    // 在 Rust 中，有两个self，一个指代当前的实例对象，一个指代特征或者方法类型的别名：
    trait Draw2 {
        fn draw(&self) -> Self;
    }
    #[derive(Clone)]
    struct Button1;
    impl Draw2 for Button1 {
        fn draw(&self) -> Self {
            return self.clone();
        }
    }

    let button = Button1;
    let newb = button.draw();
    // 上述代码中，self指代的就是当前的实例对象，也就是 button.draw() 中的 button 实例，Self 则指代的是 Button 类型。

    // 特征对象的限制
    // 1. 方法的返回类型不能是 Self
    // 2. 方法没有任何泛型参数

    // 对象安全对于特征对象是必须的，因为一旦有了特征对象，就不再需要知道实现该特征的具体类型是什么了。如果特征方法返回了具体的 Self 类型，但是特征对象忘记了其真正的类型，那这个 Self 就非常尴尬，因为没人知道它是谁了。但是对于泛型类型参数来说，当使用特征时其会放入具体的类型参数：此具体类型变成了实现该特征的类型的一部分。而当使用特征对象时其具体类型被抹去了，故而无从得知放入泛型参数类型到底是什么。
}
