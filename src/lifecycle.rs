pub fn lifecycle() {
    // 生命周期标注语法
    // 生命周期标注并不会改变任何引用的实际作用域
    // 在通过函数签名指定生命周期参数时，我们并没有改变传入引用或者返回引用的真实生命周期，而是告诉编译器当不满足此约束条件时，就拒绝编译通过

    // 函数签名中的生命周期标注
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // 需要注意的点如下：
    // 1. 和泛型一样，使用生命周期参数，需要先声明 <'a>
    // 2. x、y 和返回值至少活得和 'a 一样久(因为返回值要么是 x，要么是 y)
    // 该函数签名表明对于某些生命周期 'a，函数的两个参数都至少跟 'a 活得一样久，同时函数的返回引用也至少跟 'a 活得一样久。实际上，这意味着返回值的生命周期与参数生命周期中的较小值一致：虽然两个参数的生命周期都是标注了 'a，但是实际上这两个参数的真实生命周期可能是不一样的(生命周期 'a 不代表生命周期等于 'a，而是大于等于 'a)。

    // 结构体中的生命周期
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // ImportantExcerpt 结构体中有一个引用类型的字段 part，因此需要为它标注上生命周期。结构体的生命周期标注语法跟泛型参数语法很像，需要对生命周期参数进行声明 <'a>。该生命周期标注说明，结构体 ImportantExcerpt 所引用的字符串 str 必须比该结构体活得更久。

    // 生命周期消除
    // 在开始之前有几点需要注意：
    // 1. 消除规则不是万能的，若编译器不能确定某件事是正确时，会直接判为不正确，那么你还是需要手动标注生命周期
    // 2. 函数或者方法中，参数的生命周期被称为 输入生命周期，返回值的生命周期被称为 输出生命周期

    // 三条消除规则
    // 编译器使用三条消除规则来确定哪些场景不需要显式地去标注生命周期。其中第一条规则应用在输入生命周期上，第二、三条应用在输出生命周期上。若编译器发现三条规则都不适用时，就会报错，提示你需要手动标注生命周期。

    // 1. 每一个引用参数都会获得独自的生命周期
    // 例如一个引用参数的函数就有一个生命周期标注: fn foo<'a>(x: &'a i32)，两个引用参数的有两个生命周期标注:fn foo<'a, 'b>(x: &'a i32, y: &'b i32), 依此类推。

    // 2. 若只有一个输入生命周期(函数参数中只有一个引用类型)，那么该生命周期会被赋给所有的输出生命周期，也就是所有返回值的生命周期都等于该输入生命周期
    // 例如函数 fn foo(x: &i32) -> &i32，x 参数的生命周期会被自动赋给返回值 &i32，因此该函数等同于 fn foo<'a>(x: &'a i32) -> &'a i32

    // 3. 若存在多个输入生命周期，且其中一个是 &self 或 &mut self，则 &self 的生命周期被赋给所有的输出生命周期
    // 拥有 &self 形式的参数，说明该函数是一个 方法，该规则让方法的使用便利度大幅提升。

    // 规则其实很好理解，但是，爱思考的读者肯定要发问了，例如第三条规则，若一个方法，它的返回值的生命周期就是跟参数 &self 的不一样怎么办？总不能强迫我返回的值总是和 &self 活得一样久吧？! 问得好，答案很简单：手动标注生命周期，因为这些规则只是编译器发现你没有标注生命周期时默认去使用的，当你标注生命周期后，编译器自然会乖乖听你的话。

    // 方法中的生命周期
    struct ImportantExcerpt_1<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt_1<'a> {
        fn level(&self) -> i32 {
            3
        }
    }

    // 其中有几点需要注意的：
    // impl 中必须使用结构体的完整名称，包括 <'a>，因为生命周期标注也是结构体类型的一部分！
    // 方法签名中，往往不需要标注生命周期，得益于生命周期消除的第一和第三规则

    struct ImportantExcerpt_2<'a> {
        part: &'a str,
    }

    // 有一点很容易推理出来：由于 &'a self 是被引用的一方，因此引用它的 &'b str 必须要活得比它短，否则会出现悬垂引用。因此说明生命周期 'b 必须要比 'a 小，只要满足了这一点，编译器就不会再报错：
    impl<'a: 'b, 'b> ImportantExcerpt<'a> {
        fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    impl<'a, 'b: 'a> ImportantExcerpt_2<'a> {
        fn announce_and_return_part(&'a self, announcement: &'b str) -> &'a str {
            println!("Attention please: {}", announcement);
            announcement
        }
    }
}
