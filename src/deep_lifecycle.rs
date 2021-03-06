pub fn deep_lifecycle() {
    // 无界生命周期
    // 不安全代码(unsafe)经常会凭空产生引用或生命周期，这些生命周期被称为是 无界(unbound) 的。
    // 无界生命周期往往是在解引用一个裸指针(裸指针 raw pointer)时产生的，换句话说，它是凭空产生的，因为输入参数根本就没有这个生命周期：

    fn f<'a, T>(x: *const T) -> &'a T {
        unsafe { &*x }
    }

    // 上述代码中，参数 x 是一个裸指针，它并没有任何生命周期，然后通过 unsafe 操作后，它被进行了解引用，变成了一个 Rust 的标准引用类型，该类型必须要有生命周期，也就是 'a。
    // 可以看出 'a 是凭空产生的，因此它是无界生命周期。这种生命周期由于没有受到任何约束，因此它想要多大就多大，这实际上比 'static 要强大。例如 &'static &'a T 是无效类型，但是无界生命周期 &'unbounded &'a T 会被视为 &'a &'a T 从而通过编译检查，因为它可大可小，就像孙猴子的金箍棒一般。
    // 我们在实际应用中，要尽量避免这种无界生命周期。最简单的避免无界生命周期的方式就是在函数声明中运用生命周期消除规则。若一个输出生命周期被消除了，那么必定因为有一个输入生命周期与之对应。

    // 生命周期约束 HRTB
    // 生命周期约束跟特征约束类似，都是通过形如 'a: 'b 的语法，来说明两个生命周期的长短关系。
    // 'a: 'b
    // 假设有两个引用 &'a i32 和 &'b i32，它们的生命周期分别是 'a 和 'b，若 'a >= 'b，则可以定义 'a:'b，表示 'a 至少要活得跟 'b 一样久。

    struct DoubleRef<'a, 'b: 'a, T> {
        r: &'a T,
        s: &'b T,
    }
    // 例如上述代码定义一个结构体，它拥有两个引用字段，类型都是泛型 T，每个引用都拥有自己的生命周期，由于我们使用了生命周期约束 'b: 'a，因此 'b 必须活得比 'a 久，也就是结构体中的 s 字段引用的值必须要比 r 字段引用的值活得要久。

    // T: 'a
    // 表示类型 T 必须比 'a 活得要久：
    struct Ref<'a, T: 'a> {
        r: &'a T,
    }
    // 因为结构体字段 r 引用了 T，因此 r 的生命周期 'a 必须要比 T 的生命周期更短(被引用者的生命周期必须要比引用长)。
    // 在 Rust 1.30 版本之前，该写法是必须的，但是从 1.31 版本开始，编译器可以自动推导 T: 'a 类型的约束，因此我们只需这样写即可：
    struct Ref1<'a, T> {
        r: &'a T,
    }

    // 来看一个使用了生命周期约束的综合例子：
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a: 'b, 'b> ImportantExcerpt<'a> {
        fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }
    // 上面的例子中必须添加约束 'a: 'b 后，才能成功编译，因为 self.part 的生命周期与 self的生命周期一致，将 &'a 类型的生命周期强行转换为 &'b 类型，会报错，只有在 'a >= 'b 的情况下，'a 才能转换成 'b。

    // 闭包函数的消除规则
    fn fn_elision(x: &i32) -> &i32 {
        x
    }
    // let closure_slision = |x: &i32| -> &i32 { x };
    // 这个问题，可能很难被解决，建议大家遇到后，还是老老实实用正常的函数，不要秀闭包了。

    // NLL (Non-Lexical Lifetime)
    // 之前我们在引用与借用那一章其实有讲到过这个概念，简单来说就是：引用的生命周期正常来说应该从借用开始一直持续到作用域结束，但是这种规则会让多引用共存的情况变得更复杂：
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // 新编译器中，r1,r2作用域在这里结束

    let r3 = &mut s;
    println!("{}", r3);
    // 按照上述规则，这段代码将会报错，因为 r1 和 r2 的不可变引用将持续到 main 函数结束，而在此范围内，我们又借用了 r3 的可变引用，这违反了借用的规则：要么多个不可变借用，要么一个可变借用。
    // 好在，该规则从 1.31 版本引入 NLL 后，就变成了：引用的生命周期从借用处开始，一直持续到最后一次使用的地方。
    // 按照最新的规则，我们再来分析一下上面的代码。r1 和 r2 不可变借用在 println! 后就不再使用，因此生命周期也随之结束，那么 r3 的借用就不再违反借用的规则，皆大欢喜。

    // Reborrow 再借用
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        fn move_to(&mut self, x: i32, y: i32) {
            self.x = x;
            self.y = y;
        }
    }

    let mut p = Point { x: 0, y: 0 };
    let r = &mut p;
    let rr: &Point = &*r;

    println!("{:?}", rr);
    r.move_to(10, 10);
    println!("{:?}", r);

    // 以上代码，大家可能会觉得可变引用 r 和不可变引用 rr 同时存在会报错吧？但是事实上并不会，原因在于 rr 是对 r 的再借用。
    // 对于再借用而言，rr 再借用时不会破坏借用规则，但是你不能在它的生命周期内再使用原来的借用 r，来看看对上段代码的分析：

    // 如上所示，函数体内对参数的二次借用也是典型的 Reborrow 场景。
    fn read_length(strings: &mut Vec<String>) -> usize {
        strings.len()
    }

    // 生命周期消除规则补充
    // 在上一节中，我们介绍了三大基础生命周期消除规则，实际上，随着 Rust 的版本进化，该规则也在不断演进，这里再介绍几个常见的消除规则：

    // impl 块消除
    trait Reader {}
    struct BufReader<'a> {
        a: &'a i32,
    };
    // impl<'a> Reader for BufReader<'a> {
    //     // methods go here
    //     // impl内部实际上没有用到'a
    // }

    // 如果你以前写的impl块长上面这样，同时在 impl 内部的方法中，根本就没有用到 'a，那就可以写成下面的代码形式

    impl Reader for BufReader<'_> {
        // methods go here
    }

    // '_ 生命周期表示 BufReader 有一个不使用的生命周期，我们可以忽略它，无需为它创建一个名称。
    // 歪个楼，有读者估计会发问：既然用不到 'a，为何还要写出来？如果你仔细回忆下上一节的内容，里面有一句专门用粗体标注的文字：生命周期参数也是类型的一部分，因此 BufReader<'a> 是一个完整的类型，在实现它的时候，你不能把 'a 给丢了！

    // 生命周期约束消除

    // Rust 2015
    struct Ref2<'a, T: 'a> {
        field: &'a T,
    }

    // Rust 2018
    struct Ref3<'a, T> {
        field: &'a T,
    }
}
