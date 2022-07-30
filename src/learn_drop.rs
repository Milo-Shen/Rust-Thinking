pub fn learn_drop() {
    // 在 Rust 中，我们之所以可以一拳打跑 GC 的同时一脚踢翻手动资源回收，主要就归功于 Drop 特征，同时它也是智能指针的必备特征之一。
    // Rust 中的资源回收

    // 一个不那么简单的 Drop 例子:
    struct HasDrop1;
    struct HasDrop2;
    impl Drop for HasDrop1 {
        fn drop(&mut self) {
            println!("Dropping HasDrop1!");
        }
    }
    impl Drop for HasDrop2 {
        fn drop(&mut self) {
            println!("Dropping HasDrop2!");
        }
    }
    struct HasTwoDrops {
        one: HasDrop1,
        two: HasDrop2,
    }
    impl Drop for HasTwoDrops {
        fn drop(&mut self) {
            println!("Dropping HasTwoDrops!");
        }
    }

    struct Foo;

    impl Drop for Foo {
        fn drop(&mut self) {
            println!("Dropping Foo!")
        }
    }

    let _x = HasTwoDrops {
        two: HasDrop2,
        one: HasDrop1,
    };
    let _foo = Foo;
    println!("Running!");

    // 上面代码虽然长，但是目的其实很单纯，就是为了观察不同情况下变量级别的、结构体内部字段的 Drop，有几点值得注意：
    // Drop 特征中的 drop 方法借用了目标的可变引用，而不是拿走了所有权，这里先设置一个悬念，后边会讲
    // 结构体中每个字段都有自己的 Drop

    // Drop 的顺序
    // 观察以上输出，我们可以得出以下关于 Drop 顺序的结论
    // 1. 变量级别，按照逆序的方式，_x 在 _foo 之前创建，因此 _x 在 _foo 之后被 drop
    // 2. 结构体内部，按照顺序的方式，结构体 _x 中的字段按照定义中的顺序依次 drop
}
