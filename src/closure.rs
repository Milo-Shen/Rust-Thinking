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
}
