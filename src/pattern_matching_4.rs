#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

pub fn pattern_matching_4() {
    // 匹配守卫提供的额外条件
    // 匹配守卫（match guard）是一个位于 match 分支模式之后的额外 if 条件，它能为分支模式提供更进一步的匹配条件。
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // 在之前，我们提到可以使用匹配守卫来解决模式中变量覆盖的问题，那里 match 表达式的模式中新建了一个变量而不是使用 match 之外的同名变量。
    // 内部变量覆盖了外部变量，意味着此时不能够使用外部变量的值，下面代码展示了如何使用匹配守卫修复这个问题。

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);

    // 也可以在匹配守卫中使用 或 运算符 | 来指定多个模式，同时匹配守卫的条件会作用于所有的模式。
    // 下面代码展示了匹配守卫与 | 的优先级。这个例子中看起来好像 if y 只作用于 6，但实际上匹配守卫 if y 作用于 4、5 和 6 ，在满足 x 属于 4 | 5 | 6 后才会判断 y 是否为 true
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @绑定
    // @（读作 at）运算符允许为一个字段绑定另外一个变量。
    // 下面例子中，我们希望测试 Message::Hello 的 id 字段是否位于 3..=7 范围内，同时也希望能将其值绑定到 id_variable 变量中以便此分支中相关的代码可以使用它。
    // 我们可以将 id_variable 命名为 id，与字段同名，不过出于示例的目的这里选择了不同的名称。
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 7 };

    match msg {
        // 上例会打印出 Found an id in range: 5。通过在 3..=7 之前指定 id_variable @，我们捕获了任何匹配此范围的值并同时将该值绑定到变量 id_variable 上。
        Message::Hello {
            //    // 当你既想要限定分支范围，又想要使用分支的变量时，就可以用 @ 来绑定到一个新的变量上，实现想要的功能。
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        // 第二个分支只在模式中指定了一个范围，id 字段的值可以是 10、11 或 12，不过这个模式的代码并不知情也不能使用 id 字段中的值，因为没有将 id 值保存进一个变量。
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        // 最后一个分支指定了一个没有范围的变量，此时确实拥有可以用于分支代码的变量 id，因为这里使用了结构体字段简写语法。不过此分支中没有像头两个分支那样对 id 字段的值进行测试：任何值都会匹配此分支。
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }

    // 绑定新变量 `p`，同时对 `Point` 进行解构
    let p @ Point { x: px, y: py } = Point { x: 10, y: 23 };
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);

    let point = Point { x: 10, y: 5 };
    if let p @ Point { x: 10, y } = point {
        println!("x is 10 and y is {} in {:?}", y, p);
    } else {
        println!("x was not 10 :(");
    }

    match 1 {
        num @ (1 | 2) => {
            println!("{}", num);
        }
        _ => {}
    }
}
