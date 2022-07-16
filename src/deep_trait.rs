pub fn deep_trait() {
    // 关联类型
    // 关联类型是在特征定义的语句块中，申明一个自定义类型，这样就可以在特征的方法签名中使用该类型：
    pub trait Iterator {
        // Item 就是关联类型，用于替代遍历的值的类型。
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }

    //  Self 用来指代当前调用者的具体类型，那么 Self::Item 就用来指代该类型实现中定义的 Item 类型：
    #[derive(Debug)]
    struct Counter {}

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            Some(0)
        }
    }

    let mut c = Counter {};
    println!("c next: {:?}", c.next());
}
