pub fn trait_obj() {
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
}
