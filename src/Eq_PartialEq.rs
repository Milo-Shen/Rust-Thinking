use std::fmt::Display;

pub fn eq_partialeq() {
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

    // 部分相等性
    // 首先我们需要找到一个类型，它实现了 PartialEq 但是没有实现 Eq（你可能会想有没有反过来的情况？当然没有啦，部分相等肯定是全部相等的子集！）
    // 在 HashMap 章节提到过 HashMap 的 key 要求实现 Eq 特征，也就是要能完全相等，而浮点数由于没有实现 Eq ，因此不能用于 HashMap 的 key。
    let f1 = f32::NAN;
    let f2 = f32::NAN;

    if f1 == f2 {
        println!("NaN 竟然可以比较，这很不数学啊！")
    } else {
        println!("果然，虽然两个都是 NaN ，但是它们其实并不相等")
    }

    // 既然浮点数有一个值不可以比较相等性，那它自然只能实现 PartialEq 而不能实现 Eq 了，以此类推，如果我们的类型也有这种特殊要求，那也应该这么作。

    // Ord 和 PartialOrd
    // 事实上，还有一对与 Eq/PartialEq 非常类似的特征，它们可以用于 <、<=、> 和 >= 比较，至于哪个类型实现了 PartialOrd 却没有实现 Ord 就交给大家自己来思考了：）
}
