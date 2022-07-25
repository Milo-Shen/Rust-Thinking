use std::fmt;

pub fn newtype_and_type_aliases() {
    // newtype
    // 何为 newtype？简单来说，就是使用元组结构体的方式将已有的类型包裹起来：struct Meters(u32);，那么此处 Meters 就是一个 newtype。
    // 为何需要 newtype？Rust 这多如繁星的 Old 类型满足不了我们吗？这是因为：
    // 1. 自定义类型可以让我们给出更有意义和可读性的类型名，例如与其使用 u32 作为距离的单位类型，我们可以使用 Meters，它的可读性要好得多
    // 2. 对于某些场景，只有 newtype 可以很好地解决
    // 3. 隐藏内部类型的细节

    // 为外部类型实现外部特征
    // 在之前的章节中，我们有讲过，如果在外部类型上实现外部特征必须使用 newtype 的方式，否则你就得遵循孤儿规则：要为类型 A 实现特征 T，那么 A 或者 T 必须至少有一个在当前的作用范围内。
    // 例如，如果想使用 println!("{}", v) 的方式去格式化输出一个动态数组 Vec，以期给用户提供更加清晰可读的内容，那么就需要为 Vec 实现 Display 特征，但是这里有一个问题： Vec 类型定义在标准库中，Display 亦然，这时就可以祭出大杀器 newtype 来解决：
    struct Wrapper(Vec<String>);
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
