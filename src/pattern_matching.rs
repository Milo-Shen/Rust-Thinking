pub enum Direction {
    East,
    West,
    North,
    South,
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

pub fn pattern_matching() {
    // 在 Rust 中，模式匹配最常用的就是 match 和 if let，本章节将对两者及相关的概念进行详尽介绍。
    let dire = Direction::South;

    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        }
        _ => println!("West"),
    };

    // match 的匹配必须要穷举出所有可能，因此这里用 _ 来代表未列出的所有可能性
    // match 的每一个分支都必须是一个表达式，且所有分支的表达式最终返回值的类型必须相同
    // X | Y，类似逻辑运算符 或，代表该分支可以匹配 X 也可以匹配 Y，只要满足一个即可

    // 每个分支相关联的代码是一个表达式，而表达式的结果值将作为整个 match 表达式的返回值。如果分支有多行代码，那么需要用 {} 包裹，同时最后一行代码需要是一个表达式。

    // 模式绑定
    // 模式匹配的另外一个重要功能是从模式中取出绑定的值，例如:

    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, 0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            }
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            }
            Action::ChangeColorRGB(r, g, _) => {
                println!(
                    "change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g,
                );
            }
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // 25美分硬币
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
