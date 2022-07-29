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
}
