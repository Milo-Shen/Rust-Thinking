use std::fmt::Display;

pub fn Eq_PartialEq() {
    // Eq 和 PartialEq
    // 在 Rust 中，想要重载操作符，你就需要实现对应的特征。
    // 例如 <、<=、> 和 >= 需要实现 PartialOrd 特征:

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    let a = Pair::new(1, 1);
    a.cmp_display();
}
