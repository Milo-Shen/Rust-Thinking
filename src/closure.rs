use std::thread;
use std::time::Duration;

pub fn closure() {
    // 闭包是一种匿名函数，它可以赋值给变量也可以作为参数传递给其它函数，不同于函数的是，它允许捕获调用者作用域中的值，例如：
    let x = 1;
    let sum = |y| x + y;
    let total = sum(1);
    println!("total = {total}");

    // 上面的代码展示了非常简单的闭包 sum，它拥有一个入参 y，同时捕获了作用域中的 x 的值，因此调用 sum(2) 意味着将 2（参数 y）跟 1（x）进行相加,最终返回它们的和：3。

    // 使用闭包来简化代码
    // 传统函数实现

    // 开始健身，好累，我得发出声音：muuuu...
    fn muuuuu(intensity: u32) -> u32 {
        println!("muuuu.....");
        thread::sleep(Duration::from_millis(1));
        intensity
    }

    fn workout(intensity: u32, random_number: u32) {
        if intensity < 25 {
            println!("今天活力满满，先做 {} 个俯卧撑!", muuuuu(intensity));
            println!(
                "旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!",
                muuuuu(intensity)
            );
        } else if random_number == 3 {
            println!("昨天练过度了，今天还是休息下吧！");
        } else {
            println!(
                "昨天练过度了，今天干干有氧，跑步 {} 分钟!",
                muuuuu(intensity)
            );
        }
    }

    // 强度
    let intensity = 10;
    // 随机值用来决定某个选择
    let random_number = 7;

    // 开始健身
    workout(intensity, random_number);

    // 函数变量实现
    fn workout1(intensity: u32, random_number: u32) {
        let action = muuuuu;
        if intensity < 25 {
            println!("今天活力满满, 先做 {} 个俯卧撑!", action(intensity));
            println!(
                "旁边有妹子在看，俯卧撑太low, 再来 {} 组卧推!",
                action(intensity)
            );
        } else if random_number == 3 {
            println!("昨天练过度了，今天还是休息下吧！");
        } else {
            println!(
                "昨天练过度了，今天干干有氧, 跑步 {} 分钟!",
                action(intensity)
            );
        }
    }

    // 强度
    let intensity = 10;
    // 随机值用来决定某个选择
    let random_number = 7;

    // 开始健身
    workout1(intensity, random_number);

    // 闭包实现
    // 上面提到 intensity 要是变化怎么办，简单，使用闭包来捕获它，这是我们的拿手好戏：
    fn workout2(intensity: u32, random_number: u32) {
        let action = || {
            println!("muuuu.....");
            thread::sleep(Duration::from_millis(1));
            intensity
        };

        if intensity < 25 {
            println!("今天活力满满，先做 {} 个俯卧撑!", action());
            println!("旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!", action());
        } else if random_number == 3 {
            println!("昨天练过度了，今天还是休息下吧！");
        } else {
            println!("昨天练过度了，今天干干有氧，跑步 {} 分钟!", action());
        }
    }
    // 动作次数
    let intensity = 10;
    // 随机值用来决定某个选择
    let random_number = 7;
    // 开始健身
    workout2(intensity, random_number);

    // 上例中还有两点值得注意：
    // 1. 闭包中最后一行表达式返回的值，就是闭包执行后的返回值，因此 action() 调用返回了 intensity 的值 10
    // 2. let action = ||... 只是把闭包赋值给变量 action，并不是把闭包执行后的结果赋值给 action，因此这里 action 就相当于闭包函数，可以跟函数一样进行调用：action()

    // 闭包并不会作为 API 对外提供，因此它可以享受编译器的类型推导能力，无需标注参数和返回值的类型。
    // 为了增加代码可读性，有时候我们会显式地给类型进行标注，出于同样的目的，也可以给闭包标注类型：
    let sum = |x: i32, y: i32| -> i32 { x + y };

    // 与之相比，不标注类型的闭包声明会更简洁些：let sum = |x, y| x + y，需要注意的是，针对 sum 闭包，如果你只进行了声明，但是没有使用，编译器会提示你为 x, y 添加类型标注，因为它缺乏必要的上下文：
    let sum = |x, y| x + y;
    let v = sum(1, 2);

    // 下面展示了同一个功能的函数和闭包实现形式：
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    let add_one_v4 = |x| x + 1;
    add_one_v3(2);
    add_one_v4(2);
    // 可以看出第一行的函数和后面的闭包其实在形式上是非常接近的，同时三种不同的闭包也展示了三种不同的使用方式：省略参数、返回值类型和花括号对。

    // 虽然类型推导很好用，但是它不是泛型，当编译器推导出一种类型后，它就会一直使用该类型：
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    // let n = example_closure(5);
    // 首先，在 s 中，编译器为 x 推导出类型 String，但是紧接着 n 试图用 5 这个整型去调用闭包，跟编译器之前推导的 String 类型不符，因此报错

    // 结构体中的闭包
    struct Cacher<T>
    where
        T: Fn(T) -> T,
    {
        query: T,
        value: Option<T>,
    }
    // 其实，可以看得出这一长串是 T 的特征约束，再结合之前的已知信息：query 是一个闭包，大概可以推测出，Fn(u32) -> u32 是一个特征，用来表示 T 是一个闭包类型？Bingo，恭喜你，答对了！
    // 那为什么不用具体的类型来标注 query 呢？原因很简单，每一个闭包实例都有独属于自己的类型，即使于两个签名一模一样的闭包，它们的类型也是不同的，因此你无法用一个统一的类型来标注 query 闭包。
    // 而标准库提供的 Fn 系列特征，再结合特征约束，就能很好的解决了这个问题. T: Fn(u32) -> u32 意味着 query 的类型是 T，该类型必须实现了相应的闭包特征 Fn(u32) -> u32。从特征的角度来看它长得非常反直觉，但是如果从闭包的角度来看又极其符合直觉，不得不佩服 Rust 团队的鬼才设计。。。
    // 特征 Fn(u32) -> u32 从表面来看，就对闭包形式进行了显而易见的限制：该闭包拥有一个u32类型的参数，同时返回一个u32类型的值。
    // 需要注意的是，其实 Fn 特征不仅仅适用于闭包，还适用于函数，因此上面的 query 字段除了使用闭包作为值外，还能使用一个具名的函数来作为它的值

    struct Cacher1<T, E>
    where
        T: Fn(E) -> E,
        E: Copy,
    {
        query: T,
        value: Option<E>,
    }

    impl<T, E> Cacher1<T, E>
    where
        T: Fn(E) -> E,
        E: Copy,
    {
        fn new(query: T) -> Cacher1<T, E> {
            Cacher1 { query, value: None }
        }

        fn value(&mut self, arg: E) -> E {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.query)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }
    let mut c = Cacher1::new(|a| a);
    let v1 = c.value(1);
    let v2 = c.value(2);
    assert_eq!(v2, 1);

    // 捕获作用域中的值
    // 在之前代码中，我们一直在用闭包的匿名函数特性（赋值给变量），然而闭包还拥有一项函数所不具备的特性：捕获作用域中的值。
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    // 闭包对内存的影响
    // 当闭包从环境中捕获一个值时，会分配内存去存储这些值。对于有些场景来说，这种额外的内存分配会成为一种负担。与之相比，函数就不会去捕获这些环境值，因此定义和使用函数不会拥有这种内存负担。

    // 三种 Fn 特征
    // 闭包捕获变量有三种途径，恰好对应函数参数的三种传入方式：转移所有权、可变借用、不可变借用，因此相应的 Fn 特征也有三种：
    // 1. FnOnce，该类型的闭包会拿走被捕获变量的所有权。Once 顾名思义，说明该闭包只能运行一次：
    fn fn_once<F>(func: F)
    where
        F: FnOnce(usize) -> bool,
    {
        println!("{}", func(3));
        // 仅实现 FnOnce 特征的闭包在调用时会转移所有权，所以显然不能对已失去所有权的闭包变量进行二次调用：
        // println!("{}", func(4));
    }
    let x = vec![1, 2, 3];
    fn_once(|z| z == x.len());
    println!("{:?}", x);

    // 上面的 x 还能移动是因为实现了不可变借用, 而没有转移所有权
    // 下面的 fn_once_1 是实实在在转移了所有权，所以 x 不能再用
    fn fn_once_1<F>(func: F)
    where
        F: FnOnce() -> Vec<i32>,
    {
        println!("{:?}", func());
        // 仅实现 FnOnce 特征的闭包在调用时会转移所有权，所以显然不能对已失去所有权的闭包变量进行二次调用：
        // println!("{}", func(4));
    }
    let x = vec![1, 2, 3];
    fn_once_1(|| x);
    // println!("{:?}", x);

    // 这里面有一个很重要的提示，因为 F 没有实现 Copy 特征，所以会报错，那么我们添加一个约束，试试实现了 Copy 的闭包：
    fn fn_once1<F>(func: F)
    where
        F: FnOnce(usize) -> bool + Copy, // 改动在这里
    {
        println!("{}", func(3));
        println!("{}", func(4));
    }
    let x = vec![1, 2, 3];
    fn_once1(|z| z == x.len());
    println!("{:?}", x);

    fn fn_once2<F>(func: F)
    where
        F: FnOnce(usize) -> bool + Copy, // 改动在这里
    {
        println!("{}", func(3));
        println!("{}", func(4));
    }

    // 如果你想强制闭包取得捕获变量的所有权，可以在参数列表前添加 move 关键字，这种用法通常用于闭包的生命周期大于捕获变量的生命周期时，例如将闭包返回或移入其他线程。
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();

    let c = String::from("Fn Once");
    let d = |x: String| x + "a";
    let e = d(c);
    println!("{:?}", e);

    // 2. FnMut，它以可变借用的方式捕获了环境中的值，因此可以修改该值：
    // 想要在闭包内部捕获可变借用，需要把该闭包声明为可变类型，也就是 update_string 要修改为 mut update_string：
    let mut s = String::new();
    let mut update_string = |str| s.push_str(str);
    update_string("hello");
    println!("{:?}", s);

    // 再来看一个复杂点的：
    fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
        f("hello")
    }
    let mut s = String::new();
    let update_string = |str| s.push_str(str);
    exec(update_string);
    println!("{:?}", s);

    // Fn 特征，它以不可变借用的方式捕获环境中的值 让我们把上面的代码中 exec 的 F 泛型参数类型修改为 Fn(&'a str)：
    fn exec1<'a, F: Fn(String) -> ()>(f: F) {
        f("world".to_string())
    }
    let s = "hello, ".to_string();
    let update_string = |str| println!("{},{}", s, str);
    exec(update_string);
    println!("{:?}", s);

    // move 和 Fn
    // 在上面，我们讲到了 move 关键字对于 FnOnce 特征的重要性，但是实际上使用了 move 的闭包依然可能实现了 Fn 或 FnMut 特征。
    // 因为，一个闭包实现了哪种 Fn 特征取决于该闭包如何使用被捕获的变量，而不是取决于闭包如何捕获它们。move 本身强调的就是后者，闭包如何捕获变量：
    fn exec2<F: FnOnce()>(f: F) {
        f()
    }
    let s = String::new();
    let update_string = move || println!("{}", s);
    exec2(update_string);

    // 我们在上面的闭包中使用了 move 关键字，所以我们的闭包捕获了它，但是由于闭包对 s 的使用仅仅是不可变借用，因此该闭包实际上还实现了 Fn 特征。
    // 细心的读者肯定发现我在上段中使用了一个 还 字，这是什么意思呢？因为该闭包不仅仅实现了 FnOnce 特征，还实现了 Fn 特征，将代码修改成下面这样，依然可以编译：
    fn exec3<F: Fn()>(f: F) {
        f()
    }

    // 三种 Fn 的关系
    // 实际上，一个闭包并不仅仅实现某一种 Fn 特征，规则如下：
    // 1. 所有的闭包都自动实现了 FnOnce 特征，因此任何一个闭包都至少可以被调用一次
    // 2. 没有移出所捕获变量的所有权的闭包自动实现了 FnMut 特征
    // 3. 不需要对捕获变量进行改变的闭包自动实现了 Fn 特征
    fn exec4<F: FnMut()>(mut f: F) {
        f()
    }

    fn exec5<F: Fn()>(f: F) {
        f()
    }

    fn exec6<F: FnOnce()>(f: F) {
        f()
    }
    let s = String::from("hello world");

    let update_string = || println!("{}", s);

    exec4(update_string);
    exec5(update_string);
    exec6(update_string);
}
