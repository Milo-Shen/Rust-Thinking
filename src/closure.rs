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
}
