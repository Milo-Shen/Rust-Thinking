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
}
