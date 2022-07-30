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

    #[derive(Debug)]
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

    // 没有实现 Drop 的结构体
    // 原因在于，Rust 自动为几乎所有类型都实现了 Drop 特征，因此就算你不手动为结构体实现 Drop，它依然会调用默认实现的 drop 函数，同时再调用每个字段的 drop 方法

    // 手动回收
    // 当使用智能指针来管理锁的时候，你可能希望提前释放这个锁，然后让其它代码能及时获得锁，此时就需要提前去手动 drop。 但是在之前我们提到一个悬念，Drop::drop 只是借用了目标值的可变引用，所以，就算你提前调用了 drop，后面的代码依然可以使用目标值，但是这就会访问一个并不存在的值，非常不安全，好在 Rust 会阻止你：
    let foo = Foo;
    // foo.drop();
    //  explicit destructor calls not allowed
    // println!("Running!:{:?}", foo);

    // 如上所示，编译器直接阻止了我们调用 Drop 特征的 drop 方法，原因是对于 Rust 而言，不允许显式地调用析构函数（这是一个用来清理实例的通用编程概念）。好在在报错的同时，编译器还给出了一个提示：使用 drop 函数。
    // 针对编译器提示的 drop 函数，我们可以大胆推测下：它能够拿走目标值的所有权。现在来看看这个猜测正确与否，以下是 std::mem::drop 函数的签名：
    // pub fn drop<T>(_x: T)

    // 如上所示，drop 函数确实拿走了目标值的所有权，来验证下：
    let foo = Foo;
    drop(foo);
    // 以下代码会报错：借用了所有权被转移的值
    // println!("Running!:{:?}", foo);

    // Bingo，完美拿走了所有权，而且这种实现保证了后续的使用必定会导致编译错误，因此非常安全！
    // 细心的同学可能已经注意到，这里直接调用了 drop 函数，并没有引入任何模块信息，原因是该函数在std::prelude里。

    // Drop 使用场景
    // 对于 Drop 而言，主要有两个功能：
    // 1. 回收内存资源
    // 2. 执行一些收尾工作

    // 对于第二点，在之前我们已经详细介绍过，因此这里主要对第一点进行下简单说明。
    // 在绝大多数情况下，我们都无需手动去 drop 以回收内存资源，因为 Rust 会自动帮我们完成这些工作，它甚至会对复杂类型的每个字段都单独的调用 drop 进行回收！
    // 但是确实有极少数情况，需要你自己来回收资源的，例如文件描述符、网络 socket 等，当这些值超出作用域不再使用时，就需要进行关闭以释放相关的资源，在这些情况下，就需要使用者自己来解决 Drop 的问题。

    // 互斥的 Copy 和 Drop
    // 我们无法为一个类型同时实现 Copy 和 Drop 特征。因为实现了 Copy 的特征会被编译器隐式的复制，因此非常难以预测析构函数执行的时间和频率。因此这些实现了 Copy 的类型无法拥有析构函数。
    // #[derive(Copy)]
    // struct Foo;

    // impl Drop for Foo {
    //     fn drop(&mut self) {
    //         println!("Dropping Foo!")
    //     }
    // }

    // 总结
    // Drop 可以用于许多方面，来使得资源清理及收尾工作变得方便和安全，甚至可以用其创建我们自己的内存分配器！通过 Drop 特征和 Rust 所有权系统，你无需担心之后的代码清理，Rust 会自动考虑这些问题。
    // 我们也无需担心意外的清理掉仍在使用的值，这会造成编译器错误：所有权系统确保引用总是有效的，也会确保 drop 只会在值不再被使用时被调用一次。
}
