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
}
