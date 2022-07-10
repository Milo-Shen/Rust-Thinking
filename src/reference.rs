pub fn reference() {
    {
        let mut s = String::from("hello");

        // 注意，引用的作用域 s 从创建开始，一直持续到它最后一次使用的地方，
        // 这个跟变量的作用域有所不同，变量的作用域从创建持续到某一个花括号 }
        let r1 = &s;
        let r2 = &s;
        println!("{} and {}", r1, r2);
        // 新编译器中，r1,r2作用域在这里结束

        let r3 = &mut s;
        println!("{}", r3);
    } // 老编译器中，r1、r2、r3作用域在这里结束
      // 新编译器中，r3作用域在这里结束
}

// NLL
// 对于这种编译器优化行为，Rust 专门起了一个名字 —— Non-Lexical Lifetimes(NLL)，
// 专门用于找到某个引用在作用域(})结束前就不再被使用的代码位置。
