use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

pub fn return_values_and_error_handling() {
    // 可恢复错误，通常用于从系统全局角度来看可以接受的错误，例如处理用户的访问、操作等错误，这些错误只会影响某个用户自身的操作进程，而不会对系统的全局稳定性产生影响
    // 不可恢复错误，刚好相反，该错误通常是全局性或者系统性的错误，例如数组越界访问，系统启动时发生了影响启动流程的错误等等，这些错误的影响往往对于系统来说是致命的
    // Result<T, E> 用于可恢复错误，panic! 用于不可恢复错误。

    // panic! 与不可恢复错误
    // 上面的问题在真实场景会经常遇到，其实处理起来挺复杂的，让我们先做一个假设：文件读取操作发生在系统启动阶段。那么可以轻易得出一个结论，一旦文件读取失败，那么系统启动也将失败，这意味着该失败是不可恢复的错误，无论是因为文件不存在还是操作系统硬盘的问题，这些只是错误的原因不同，但是归根到底都是不可恢复的错误(梳理清楚当前场景的错误类型非常重要)。
    // 既然是不可恢复错误，那么一旦发生，只需让程序崩溃即可。对此，Rust 为我们提供了 panic! 宏，当调用执行该宏时，程序会打印出一个错误信息，展开报错点往前的函数调用堆栈，最后退出程序。
    // 切记，一定是不可恢复的错误，才调用 panic! 处理，你总不想系统仅仅因为用户随便传入一个非法参数就崩溃吧？所以，只有当你不知道该如何处理时，再去调用 panic!.

    // panic 时的两种终止方式
    // [profile.release]
    // panic = 'abort'

    // 线程 panic 后，程序是否会终止？
    // 长话短说，如果是 main 线程，则程序会终止，如果是其它子线程，该线程会终止，但是不会影响 main 线程。因此，尽量不要在 main 线程中做太多任务，将这些任务交由子线程去做，就算子线程 panic 也不会导致整个程序的结束。

    // panic 原理剖析
    // 当调用 panic! 宏时，它会 :
    // 1. 格式化 panic 信息，然后使用该信息作为参数，调用 std::panic::panic_any() 函数
    // 2. panic_any 会检查应用是否使用了 panic hook，如果使用了，该 hook 函数就会被调用（hook 是一个钩子函数，是外部代码设置的，用于在 panic 触发时，执行外部代码所需的功能）
    // 3. 当 hook 函数返回后，当前的线程就开始进行栈展开：从 panic_any 开始，如果寄存器或者栈因为某些原因信息错乱了，那很可能该展开会发生异常，最终线程会直接停止，展开也无法继续进行
    // 4. 展开的过程是一帧一帧的去回溯整个栈，每个帧的数据都会随之被丢弃，但是在展开过程中，你可能会遇到被用户标记为 catching 的帧（通过 std::panic::catch_unwind() 函数标记），此时用户提供的 catch 函数会被调用，展开也随之停止：当然，如果 catch 选择在内部调用 std::panic::resume_unwind() 函数，则展开还会继续。

    // 还有一种情况，在展开过程中，如果展开本身 panic 了，那展开线程会终止，展开也随之停止。
    // 一旦线程展开被终止或者完成，最终的输出结果是取决于哪个线程 panic：对于 main 线程，操作系统提供的终止功能 core::intrinsics::abort() 会被调用，最终结束当前的 panic 进程；如果是其它子线程，那么子线程就会简单的终止，同时信息会在稍后通过 std::thread::join() 进行收集。

    // 可恢复的错误 Result

    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    // 泛型参数 T 代表成功时存入的正确值的类型，存放方式是 Ok(T)，E 代表错误是存入的错误值，存放方式是 Err(E)，枯燥的讲解永远不及代码生动准确，因此先来看下打开文件的例子：

    // 如何获知变量类型或者函数的返回类型
    // 有几种常用的方式，此处更推荐第二种方法：
    // 1. 第一种是查询标准库或者三方库文档，搜索 File，然后找到它的 open 方法
    // 2. 在 Rust IDE 章节，我们推荐了 VSCode IDE 和 rust-analyzer 插件，如果你成功安装的话，那么就可以在 VSCode 中很方便的通过代码跳转的方式查看代码，同时 rust-analyzer 插件还会对代码中的类型进行标注，非常方便好用！
    // 3. 你还可以尝试故意标记一个错误的类型，然后让编译器告诉你：

    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error)
    //     }
    // };

    // 对返回的错误进行处理
    // 直接 panic 还是过于粗暴，因为实际上 IO 的错误有很多种，我们需要对部分错误进行特殊处理，而不是所有错误都直接崩溃：

    let f = File::open("hello.txt");
    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    // 上面代码在匹配出 error 后，又对 error 进行了详细的匹配解析，最终结果：
    // 如果是文件不存在错误 ErrorKind::NotFound，就创建文件，这里创建文件File::create 也是返回 Result，因此继续用 match 对其结果进行处理：创建成功，将新的文件句柄赋值给 f，如果失败，则 panic
    // 剩下的错误，一律 panic

    // 失败就 panic: unwrap 和 expect
    // 在不需要处理错误的场景，例如写原型、示例时，我们不想使用 match 去匹配 Result<T, E> 以获取其中的 T 值，因为 match 的穷尽匹配特性，你总要去处理下 Err 分支。那么有没有办法简化这个过程？有，答案就是 unwrap 和 expect。
    let f = File::open("hello.txt").unwrap();

    // expect 跟 unwrap 很像，也是遇到错误直接 panic, 但是会带上自定义的错误提示信息，相当于重载了错误打印的函数：
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // 传播错误
    // 咱们的程序几乎不太可能只有 A->B 形式的函数调用，一个设计良好的程序，一个功能涉及十几层的函数调用都有可能。而错误处理也往往不是哪里调用出错，就在哪里处理，实际应用中，大概率会把错误层层上传然后交给调用链的上游函数进行处理，错误传播将极为常见。
    // 例如以下函数从文件中读取用户名，然后将结果进行返回：
    fn read_username_from_file() -> Result<String, io::Error> {
        // 打开文件，f是`Result<文件句柄,io::Error>`
        let f = File::open("hello.txt");

        let mut f = match f {
            // 打开文件成功，将file句柄赋值给f
            Ok(file) => file,
            // 打开文件失败，将错误返回(向上传播)
            Err(e) => return Err(e),
        };
        // 创建动态字符串 s
        let mut s = String::new();
        // 从f文件句柄读取数据并写入s中
        match f.read_to_string(&mut s) {
            // 读取成功，返回Ok封装的字符串
            Ok(_) => Ok(s),
            // 将错误向上传播
            Err(e) => Err(e),
        }
        // 有几点值得注意：
        // 该函数返回一个 Result<String, io::Error> 类型，当读取用户名成功时，返回 Ok(String)，失败时，返回 Err(io:Error)
        // File::open 和 f.read_to_string 返回的 Result<T, E> 中的 E 就是 io::Error
    }
}
