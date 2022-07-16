use std::fmt;

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

    // 为了代码的可读性，当你使用了泛型后，你需要在所有地方都写 Iterator<Item>，而使用了关联类型，你只需要写 Iterator，当类型定义复杂时，这种写法可以极大的增加可读性：
    pub trait CacheableItem: Clone + Default + fmt::Debug {
        type Address: AsRef<[u8]> + Clone + fmt::Debug + Eq;
        fn is_null(&self) -> bool;
    }

    // 例如上面的代码，Address 的写法自然远比 AsRef<[u8]> + Clone + fmt::Debug + Eq + Hash 要简单的多，而且含义清晰。
    // 再例如，如果使用泛型，你将得到以下的代码：
    trait Container<A, B> {
        fn contains(&self, a: A, b: B) -> bool;
    }

    fn difference<A, B, C>(container: &C) -> i32
    where
        C: Container<A, B>,
    {
        1
    }

    // 可以看到，由于使用了泛型，导致函数头部也必须增加泛型的声明，而使用关联类型，将得到可读性好得多的代码：
    trait Container1 {
        type A;
        type B;
        fn contains(&self, a: Self::A, b: Self::B) -> bool;
    }

    fn difference1<C: Container1>(container: &C) {}

    trait Container2 {
        type A;
        type B;
        fn contains(&self, a: &Self::A, b: &Self::B);
    }

    struct ContainerExp<A, B> {
        a: A,
        b: B,
    }

    impl<A, B> Container2 for ContainerExp<A, B> {
        type A = u32;
        type B = u32;
        fn contains(&self, a: &Self::A, b: &Self::B) {
            println!("A: {}, B: {}", a, b);
        }
    }

    // todo: add example for this
    let container_exp_1 = ContainerExp { a: 1, b: 1 };
    container_exp_1.contains(&container_exp_1.a, &container_exp_1.b);

    // 默认泛型类型参数
    // 当使用泛型类型参数时，可以为其指定一个默认的具体类型，例如标准库中的 std::ops::Add 特征：
    trait Add<RHS = Self> {
        type Output;

        fn add(self, rhs: RHS) -> Self::Output;
    }
    // 它有一个泛型参数 RHS，但是与我们以往的用法不同，这里它给 RHS 一个默认值，也就是当用户不指定 RHS 时，默认使用两个同样类型的值进行相加，然后返回一个关联类型 Output。
}
