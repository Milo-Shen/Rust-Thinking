pub fn pattern_matching_3() {
    // 匹配字面值
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 匹配命名变量
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    // 单分支多模式
    //  match 表达式中，可以使用 | 语法匹配多个模式，它代表 或的意思。
    // 例如，如下代码将 x 的值与匹配分支相比较，第一个分支有 或 选项，意味着如果 x 的值匹配此分支的任何一个模式，它就会运行：
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 通过序列 ..= 匹配值的范围
    // ..= 语法允许你匹配一个闭区间序列内的值。在如下代码中，当模式匹配任何在此序列内的值时，该分支会执行：
    let x = 5;

    // 序列只允许用于数字或字符类型，原因是：它们可以连续，同时编译器在编译期可以检查该序列是否为空，字符和数字值是 Rust 中仅有的可以用于判断是否为空的类型。
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // 如下是一个使用字符类型序列的例子:
    // Rust 知道 'c' 位于第一个模式的序列内，所以会打印出 early ASCII letter。
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // 解构并分解值
    // 解构结构体
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    // 这段代码创建了变量 a 和 b 来匹配结构体 p 中的 x 和 y 字段，这个例子展示了模式中的变量名不必与结构体中的字段名一致。不过通常希望变量名与字段名一致以便于理解变量来自于哪些字段。
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // 因为变量名匹配字段名是常见的，同时因为 let Point { x: x, y: y } = p; 中 x 和 y 重复了，所以对于匹配结构体字段的模式存在简写：只需列出结构体字段的名称，则模式创建的变量会有相同的名称。下例与上例有着相同行为的代码，不过 let 模式创建的变量为 x 和 y 而不是 a 和 b：
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // 也可以使用字面值作为结构体模式的一部分进行解构，而不是为所有的字段创建变量。这允许我们测试一些字段为特定值的同时创建其他字段的变量。
    // 下文展示了固定某个字段的匹配方式:
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }

    // 解构嵌套的结构体和枚举
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message1 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message1::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message1::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message1::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            )
        }
        _ => (),
    }

    // 解构结构体和元组
    // 我们甚至可以用复杂的方式来混合、匹配和嵌套解构模式。如下是一个复杂结构体的例子，其中结构体和元组嵌套在元组中，并将所有的原始类型解构出来：
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("{}, {}, {}, {}", feet, inches, x, y);

    // 使用 _ 忽略整个值
    // 虽然 _ 模式作为 match 表达式最后的分支特别有用，但是它的作用还不限于此。例如可以将其用于函数参数中：
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }
    foo(3, 4);

    // 使用嵌套的 _ 忽略部分值
    // 可以在一个模式内部使用 _ 忽略部分值:
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    // 第一个匹配分支，我们不关心里面的值，只关心元组中两个元素的类型，因此对于 Some 中的值，直接进行忽略。 剩下的形如 (Some(_),None)，(None, Some(_)), (None,None) 形式，都由第二个分支 _ 进行分配。
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    // 还可以在一个模式中的多处使用下划线来忽略特定值，如下所示，这里忽略了一个五元元组中的第二和第四个值:
    let numbers = (2, 4, 8, 16, 32);

    // 老生常谈：模式匹配一定要类型相同，因此匹配 numbers 元组的模式，也必须有五个值（元组中元素的数量也属于元组类型的一部分）。
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }
}
