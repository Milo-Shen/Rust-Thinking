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

    // 再比如， + 号需要实现 std::ops::Add 特征，而本文的主角 Eq 和 PartialEq 正是 == 和 != 所需的特征，那么问题来了，这两个特征有何区别？
    enum BookFormat {
        Paperback,
        Hardback,
        Ebook,
    }
    struct Book {
        isbn: i32,
        format: BookFormat,
    }

    impl PartialEq for Book {
        fn eq(&self, other: &Self) -> bool {
            self.isbn == other.isbn
        }
    }

    impl Eq for Book {}

    // 其实，关键点就在于 partial 上，如果我们的类型只在部分情况下具有相等性，那你就只能实现 PartialEq，否则可以实现 PartialEq 然后再默认实现 Eq。
    // 好的，问题逐步清晰起来，现在我们只需要搞清楚何为部分相等。
}
