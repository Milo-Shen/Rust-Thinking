pub fn learning_string() {
    // Rust 中的字符是 Unicode 类型，因此每个字符占据 4 个字节内存空间，
    // 但是在字符串中不一样，字符串是 UTF-8 编码，也就是字符串中的字符所占的字节数是变化的(1 - 4)
    // 当 Rust 用户提到字符串时，往往指的就是 String 类型和 &str 字符串切片类型，这两个类型都是 UTF-8 编码。
    let s = String::from("hello,world!");
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());

    // 如果问你该字符串多长，你可能会说 3，但是实际上是 9 个字节的长度，
    // 因为大部分常用汉字在 UTF-8 中的长度是 3 个字节，因此这种情况下对 hello 进行索引，访问 &hello[0] 没有任何意义，
    // 因为你取不到 中 这个字符，而是取到了这个字符三个字节中的第一个字节，这是一个非常奇怪而且难以理解的返回值。
    let hello = String::from("中国人");
    println!("{}", hello);

    let a = String::from("नमस्ते");
    let a_vec: Vec<char> = a.chars().collect();
    // a 是以字母序的方式去看待字符串, a_vec 是以字符 char 的形式去看待字符串, 结果是不同的
    println!("{} ,{:?}", a, a_vec);

    // 还有一个原因导致了 Rust 不允许去索引字符串：因为索引操作，我们总是期望它的性能表现是 O(1)，
    // 然而对于 String 类型来说，无法保证这一点，因为 Rust 可能需要从 0 开始去遍历字符串来定位合法的字符。

    // 追加 (Push)
    let mut s = String::from("Hello ");
    s.push('r');
    println!("追加字符 push() -> {}", s);
    s.push_str("ust!");
    println!("追加字符串 push_str() -> {}", s);

    // 插入 (Insert)
    let mut s = String::from("Hello rust!");
    s.insert(5, ',');
    println!("插入字符 insert() -> {}", s);
    s.insert_str(6, " I like");
    println!("插入字符串 insert_str() -> {}", s);

    // 替换 ( replace )
    // replace 和 replacen: 该方法是返回一个新的字符串，而不是操作原来的字符串。
    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replace = string_replace.replace("rust", "RUST");
    dbg!(new_string_replace);

    //  // 替换 ( replacen )
    // replace 和 replacen: 该方法是返回一个新的字符串，而不是操作原来的字符串。
    let string_replace = "I like rust. Learning rust is my favorite!";
    let new_string_replacen = string_replace.replacen("rust", "RUST", 1);
    dbg!(new_string_replacen);

    // replace 和 replacen: 该方法是返回一个新的字符串，而不是操作原来的字符串。
    // replace_range: 该方法是直接操作原来的字符串，不会返回新的字符串。该方法需要使用 mut 关键字修饰。
    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);

    // 删除 (Delete)
    // 与字符串删除相关的方法有 4 个，他们分别是 pop()，remove()，truncate()，clear()。这四个方法仅适用于 String 类型。

    // pop —— 删除并返回字符串的最后一个字符
    // 该方法是直接操作原来的字符串。但是存在返回值，其返回值是一个 Option 类型，如果字符串为空，则返回 None。 示例代码如下：
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);

    // remove —— 删除并返回字符串中指定位置的字符
    // 该方法是直接操作原来的字符串。但是存在返回值，其返回值是删除位置的字符串，只接收一个参数，表示该字符起始索引位置。
    // remove() 方法是按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误。
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    // 删除第一个汉字
    string_remove.remove(0);
    // 下面代码会发生错误
    // string_remove.remove(1);
    // 直接删除第二个汉字
    // string_remove.remove(3);
    dbg!(string_remove);

    // truncate —— 删除字符串中从指定位置开始到结尾的全部字符
    // 该方法是直接操作原来的字符串。无返回值。该方法 truncate() 方法是按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误。
    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3);
    dbg!(string_truncate);

    // clear —— 清空字符串
    // 该方法是直接操作原来的字符串。调用后，删除字符串中的所有字符，相当于 truncate() 方法参数为 0 的时候。
    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);

    // 连接 (Catenate)

    // 1、使用 + 或者 += 连接字符串
    // 使用 + 或者 += 连接字符串，要求右边的参数必须为字符串的切片引用（Slice)类型。
    // 其实当调用 + 的操作符时，相当于调用了 std::string 标准库中的 add() 方法，这里 add() 方法的第二个参数是一个引用的类型。
    // 因此我们在使用 +， 必须传递切片引用类型。不能直接传递 String 类型。+ 和 += 都是返回一个新的字符串。
    // 所以变量声明可以不需要 mut 关键字修饰。
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust会自动解引用为&str
    let result = string_append + &string_rust;
    let mut result = result + "!";
    result += "!!!";
    println!("连接字符串 + -> {}", result);

    // add() 方法的定义: fn add(self, s: &str) -> String
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
    let s3 = s1 + &s2;
    assert_eq!(s3, "hello,world!");
    // 下面的语句如果去掉注释，就会报错
    // println!("{}",s1);

    // String = String + &str + &str + &str + &str
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    dbg!(s);

    // 2、使用 format! 连接字符串
    // format! 这种方式适用于 String 和 &str 。format! 的用法与 print! 的用法类似，详见格式化输出。
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);
}

fn say_hello(s: &str) {
    println!("{}", s);
}
