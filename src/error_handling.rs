use std::fmt::{self, Debug, Display};

pub fn error_handling() {
    // 在之前的返回值和错误处理章节中，我们学习了几个重要的概念，例如 Result 用于返回结果处理，? 用于错误的传播，若大家对此还较为模糊，强烈建议回头温习下。
    // 在本章节中一起来看看如何对 Result ( Option ) 做进一步的处理，以及如何定义自己的错误类型。

    // 组合器
    // 在设计模式中，有一个组合器模式，相信有 Java 背景的同学对此并不陌生。
    // 将对象组合成树形结构以表示“部分整体”的层次结构。组合模式使得用户对单个对象和组合对象的使用具有一致性。–GoF <<设计模式>>

    // 与组合起模式有所不同，在 Rust 中，组合器更多的是用于对返回结果的类型进行变换：例如使用 ok_or 将一个 Option 类型转换成 Result 类型。

    // or() 和 and()
    // 跟布尔关系的与/或很像，这两个方法会对两个表达式做逻辑组合，最终返回 Option / Result。

    // or()，表达式按照顺序求值，若任何一个表达式的结果是 Some 或 Ok，则该值会立刻返回
    // and()，若两个表达式的结果都是 Some 或 Ok，则第二个表达式中的值被返回。若任何一个的结果是 None 或 Err ，则立刻返回。
    // 实际上，只要将布尔表达式的 true / false，替换成 Some / None 或 Ok / Err 就很好理解了。

    let s1 = Some("some1");
    let s2 = Some("some2");
    let n: Option<&str> = None;

    let o1: Result<&str, &str> = Ok("ok1");
    let o2: Result<&str, &str> = Ok("ok2");
    let e1: Result<&str, &str> = Err("error1");
    let e2: Result<&str, &str> = Err("error2");

    assert_eq!(s1.or(s2), s1); // Some1 or Some2 = Some1
    assert_eq!(s1.or(n), s1); // Some or None = Some
    assert_eq!(n.or(s1), s1); // None or Some = Some
    assert_eq!(n.or(n), n); // None1 or None2 = None2

    assert_eq!(o1.or(o2), o1); // Ok1 or Ok2 = Ok1
    assert_eq!(o1.or(e1), o1); // Ok or Err = Ok
    assert_eq!(e1.or(o1), o1); // Err or Ok = Ok
    assert_eq!(e1.or(e2), e2); // Err1 or Err2 = Err2

    assert_eq!(s1.and(s2), s2); // Some1 and Some2 = Some2
    assert_eq!(s1.and(n), n); // Some and None = None
    assert_eq!(n.and(s1), n); // None and Some = None
    assert_eq!(n.and(n), n); // None1 and None2 = None1

    assert_eq!(o1.and(o2), o2); // Ok1 and Ok2 = Ok2
    assert_eq!(o1.and(e1), e1); // Ok and Err = Err
    assert_eq!(e1.and(o1), e1); // Err and Ok = Err
    assert_eq!(e1.and(e2), e1); // Err1 and Err2 = Err1

    // 除了 or 和 and 之外，Rust 还为我们提供了 xor ，但是它只能应用在 Option 上，其实想想也是这个理，如果能应用在 Result 上，那你又该如何对一个值和错误进行异或操作？

    // or_else() 和 and_then()
    // 它们跟 or() 和 and() 类似，唯一的区别在于，它们的第二个表达式是一个闭包。

    // or_else() 的例子 :
    // or_else with Option
    let s1 = Some("some1");
    let s2 = Some("some2");
    let fn_some = || Some("some2"); // 类似于: let fn_some = || -> Option<&str> { Some("some2") };

    let n: Option<&str> = None;
    let fn_none = || None;

    assert_eq!(s1.or_else(fn_some), s1); // Some1 or_else Some2 = Some1
    assert_eq!(s1.or_else(fn_none), s1); // Some or_else None = Some
    assert_eq!(n.or_else(fn_some), s2); // None or_else Some = Some
    assert_eq!(n.or_else(fn_none), None); // None1 or_else None2 = None2

    // or_else with Result
    let o1: Result<&str, &str> = Ok("ok1");
    let o2: Result<&str, &str> = Ok("ok2");
    let fn_ok = |_| Ok("ok2"); // 类似于: let fn_ok = |_| -> Result<&str, &str> { Ok("ok2") };

    let e1: Result<&str, &str> = Err("error1");
    let e2: Result<&str, &str> = Err("error2");
    let fn_err = |_| Err("error2");

    assert_eq!(o1.or_else(fn_ok), o1); // Ok1 or_else Ok2 = Ok1
    assert_eq!(o1.or_else(fn_err), o1); // Ok or_else Err = Ok
    assert_eq!(e1.or_else(fn_ok), o2); // Err or_else Ok = Ok
    assert_eq!(e1.or_else(fn_err), e2); // Err1 or_else Err2 = Err2

    // and_then() 的例子
    let s1 = Some("some1");
    let s2 = Some("some2");
    let fn_some = |_| Some("some2"); // 类似于: let fn_some = |_| -> Option<&str> { Some("some2") };

    let n: Option<&str> = None;
    let fn_none = |_| None;

    assert_eq!(s1.and_then(fn_some), s2); // Some1 and_then Some2 = Some2
    assert_eq!(s1.and_then(fn_none), n); // Some and_then None = None
    assert_eq!(n.and_then(fn_some), n); // None and_then Some = None
    assert_eq!(n.and_then(fn_none), n); // None1 and_then None2 = None1

    // and_then with Result
    let o1: Result<&str, &str> = Ok("ok1");
    let o2: Result<&str, &str> = Ok("ok2");
    let fn_ok = |_| Ok("ok2"); // 类似于: let fn_ok = |_| -> Result<&str, &str> { Ok("ok2") };

    let e1: Result<&str, &str> = Err("error1");
    let e2: Result<&str, &str> = Err("error2");
    let fn_err = |_| Err("error2");

    assert_eq!(o1.and_then(fn_ok), o2); // Ok1 and_then Ok2 = Ok2
    assert_eq!(o1.and_then(fn_err), e2); // Ok and_then Err = Err
    assert_eq!(e1.and_then(fn_ok), e1); // Err and_then Ok = Err
    assert_eq!(e1.and_then(fn_err), e1); // Err1 and_then Err2 = Err1

    // filter
    // filter 用于对 Option 进行过滤：
    let s1 = Some(3);
    let s2 = Some(6);
    let n = None;
    let fn_is_even = |x: &i8| x % 2 == 0;
    assert_eq!(s1.filter(fn_is_even), n); // Some(3) -> 3 is not even -> None
    assert_eq!(s2.filter(fn_is_even), s2); // Some(6) -> 6 is even -> Some(6)
    assert_eq!(n.filter(fn_is_even), n); // None -> no value -> None

    // map() 和 map_err()
    // map 可以将 Some 或 Ok 中的值映射为另一个：
    let s1 = Some("abcde");
    let s2 = Some(5);

    let n1: Option<&str> = None;
    let n2: Option<usize> = None;

    let o1: Result<&str, &str> = Ok("abcde");
    let o2: Result<usize, &str> = Ok(5);

    let e1: Result<&str, &str> = Err("abcde");
    let e2: Result<usize, &str> = Err("abcde");

    let fn_character_count = |s: &str| s.chars().count();

    assert_eq!(s1.map(fn_character_count), s2); // Some1 map = Some2
    assert_eq!(n1.map(fn_character_count), n2); // None1 map = None2

    assert_eq!(o1.map(fn_character_count), o2); // Ok1 map = Ok2
    assert_eq!(e1.map(fn_character_count), e2); // Err1 map = Err2

    // 但是如果你想要将 Err 中的值进行改变， map 就无能为力了，此时我们需要用 map_err：
    let o1: Result<&str, &str> = Ok("abcde");
    let o2: Result<&str, isize> = Ok("abcde");

    let e1: Result<&str, &str> = Err("404");
    let e2: Result<&str, isize> = Err(404);

    let fn_character_count = |s: &str| -> isize { s.parse().unwrap() }; // 该函数返回一个 isize

    assert_eq!(o1.map_err(fn_character_count), o2); // Ok1 map = Ok2
    assert_eq!(e1.map_err(fn_character_count), e2); // Err1 map = Err2

    // 通过对 o1 的操作可以看出，与 map 面对 Err 时的短小类似， map_err 面对 Ok 时也是相当无力的。

    // map_or() 和 map_or_else()
    // map_or 在 map 的基础上提供了一个默认值:
    const V_DEFAULT: u32 = 1;

    let s: Result<u32, ()> = Ok(10);
    let n: Option<u32> = None;
    let fn_closure = |v: u32| v + 2;

    assert_eq!(s.map_or(V_DEFAULT, fn_closure), 12);
    assert_eq!(n.map_or(V_DEFAULT, fn_closure), V_DEFAULT);
    // 如上所示，当处理 None 的时候，V_DEFAULT 作为默认值被直接返回。

    // map_or_else 与 map_or 类似，但是它是通过一个闭包来提供默认值:
    let s = Some(10);
    let n: Option<i8> = None;

    let fn_closure = |v: i8| v + 2;
    let fn_default = || 1;

    assert_eq!(s.map_or_else(fn_default, fn_closure), 12);
    assert_eq!(n.map_or_else(fn_default, fn_closure), 1);

    let o = Ok(10);
    let e = Err(5);
    let fn_default_for_result = |v: i8| v + 1; // 闭包可以对 Err 中的值进行处理，并返回一个新值

    assert_eq!(o.map_or_else(fn_default_for_result, fn_closure), 12);
    assert_eq!(e.map_or_else(fn_default_for_result, fn_closure), 6);

    // ok_or() and ok_or_else()
    // 这两兄弟可以将 Option 类型转换为 Result 类型。其中 ok_or 接收一个默认的 Err 参数:
    const ERR_DEFAULT: &str = "error message";

    let s = Some("abcde");
    let n: Option<&str> = None;

    let o: Result<&str, &str> = Ok("abcde");
    let e: Result<&str, &str> = Err(ERR_DEFAULT);

    assert_eq!(s.ok_or(ERR_DEFAULT), o); // Some(T) -> Ok(T)
    assert_eq!(n.ok_or(ERR_DEFAULT), e); // None -> Err(default)

    // 而 ok_or_else 接收一个闭包作为 Err 参数:
    let s = Some("abcde");
    let n: Option<&str> = None;
    let fn_err_message = || "error message";

    let o: Result<&str, &str> = Ok("abcde");
    let e: Result<&str, &str> = Err("error message");

    assert_eq!(s.ok_or_else(fn_err_message), o); // Some(T) -> Ok(T)
    assert_eq!(n.ok_or_else(fn_err_message), e); // None -> Err(default)

    // 自定义错误类型
    // 虽然标准库定义了大量的错误类型，但是一个严谨的项目，光使用这些错误类型往往是不够的，例如我们可能会为暴露给用户的错误定义相应的类型。
    // 为了帮助我们更好的定义错误，Rust 在标准库中提供了一些可复用的特征，例如 std::error::Error 特征：

    // pub trait Error: Debug + Display {
    //     fn source(&self) -> Option<&(Error + 'static)> { ... }
    // }

    // 当自定义类型实现该特征后，该类型就可以作为 Err 来使用，下面一起来看看。
    // 实际上，自定义错误类型只需要实现 Debug 和 Display 特征即可，source 方法是可选的，而 Debug 特征往往也无需手动实现，可以直接通过 derive 来派生

    // 最简单的错误
    // AppError 是自定义错误类型，它可以是当前包中定义的任何类型，在这里为了简化，我们使用了单元结构体作为例子。
    // 为 AppError 自动派生 Debug 特征
    #[derive(Debug)]
    struct AppError;

    // 为 AppError 实现 std::fmt::Display 特征
    impl Display for AppError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "An Error Occurred, Please Try Again!") // user-facing output
        }
    }

    // 一个示例函数用于产生 AppError 错误
    fn produce_error() -> Result<(), AppError> {
        Err(AppError)
    }

    match produce_error() {
        Err(e) => eprintln!("{}", e),
        _ => println!("No error"),
    }

    eprintln!("print = {:?}", produce_error()); // Err({ file: src/main.rs, line: 17 })

    // 上面的代码中除了实现 From 外，还有一点特别重要，那就是 ? 可以将错误进行隐式的强制转换：File::open 返回的是 std::io::Error， 我们并没有进行任何显式的转换，它就能自动变成 AppError ，这就是 ? 的强大之处！
    // 总结
    // Rust 一个为人津津乐道的点就是强大、易用的错误处理，对于新手来说，这个机制可能会有些复杂，但是一旦体会到了其中的好处，你将跟我一样沉醉其中不能自拔。
}
