pub fn self_referential_struct() {
    // 平平无奇的自引用
    // 可能也有不少人第一次听说自引用结构体，那咱们先来看看它们长啥样。
    struct SelfRef<'a> {
        value: String,
        // 该引用指向上面的value
        pointer_to_value: &'a str,
    }

    // 以上就是一个很简单的自引用结构体，看上去好像没什么，那来试着运行下：
    // let s = "aaa".to_string();
    // let v = SelfRef {
    //     value: s,
    //     pointer_to_value: &s,
    // };

    // 因为我们试图同时使用值和值的引用，最终所有权转移和借用一起发生了。所以，这个问题貌似并没有那么好解决，不信你可以回想下自己具有的知识，是否可以解决？

    // 最简单的方式就是使用 Option 分两步来实现：
    #[derive(Debug)]
    struct WhatAboutThis<'a> {
        name: String,
        nickname: Option<&'a str>,
    }
    let mut tricky = WhatAboutThis {
        name: "Annabelle".to_string(),
        nickname: None,
    };
    tricky.nickname = Some(&tricky.name[..4]);

    println!("{:?}", tricky);

    // 在某种程度上来说，Option 这个方法可以工作，但是这个方法的限制较多，例如从一个函数创建并返回它是不可能的：
    // fn creator<'a>() -> WhatAboutThis<'a> {
    //     let mut tricky = WhatAboutThis {
    //         name: "Annabelle".to_string(),
    //         nickname: None,
    //     };
    //     tricky.nickname = Some(&tricky.name[..4]);

    //     tricky
    // }

    // 其实从函数签名就能看出来端倪，'a 生命周期是凭空产生的！
    // 如果是通过方法使用，你需要一个无用 &'a self 生命周期标识，一旦有了这个标识，代码将变得更加受限，你将很容易就获得借用错误，就连 NLL 规则都没用：
    #[derive(Debug)]
    struct WhatAboutThis1<'a> {
        name: String,
        nickname: Option<&'a str>,
    }

    impl<'a> WhatAboutThis1<'a> {
        fn tie_the_knot(&'a mut self) {
            self.nickname = Some(&self.name[..4]);
        }
    }

    let mut tricky = WhatAboutThis1 {
        name: "Annabelle".to_string(),
        nickname: None,
    };
    tricky.tie_the_knot();

    // cannot borrow `tricky` as immutable because it is also borrowed as mutable
    // println!("{:?}", tricky);
    // 警惕 UTF-8 引发的性能隐患
    let s: &str = "中国人";
    for c in s.chars() {
        println!("{}", c) // 依次输出：中 、 国 、 人
    }

    let c = &s[0..3]; // 1. "中" 在 UTF-8 中占用 3 个字节 2. Rust 不支持字符串索引，因此只能通过切片的方式获取 "中"
    assert_eq!(c, "中");
}
