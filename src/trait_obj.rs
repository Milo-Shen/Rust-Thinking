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
}
