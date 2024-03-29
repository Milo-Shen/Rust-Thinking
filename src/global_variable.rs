use std::sync::atomic::{AtomicUsize, Ordering};

pub fn global_variable() {
    // 在一些场景，我们可能需要全局变量来简化状态共享的代码，包括全局 ID，全局数据存储等等，下面一起来看看有哪些创建全局变量的方法。
    // 首先，有一点可以肯定，全局变量的生命周期肯定是'static，但是不代表它需要用static来声明，例如常量、字符串字面值等无需使用static进行声明，原因是它们已经被打包到二进制可执行文件中。
    // 下面我们从编译期初始化及运行期初始化两个类别来介绍下全局变量有哪些类型及该如何使用。

    // 编译期初始化
    // 我们大多数使用的全局变量都只需要在编译期初始化即可，例如静态配置、计数器、状态值等等。

    // 静态常量
    // 全局常量可以在程序任何一部分使用，当然，如果它是定义在某个模块中，你需要引入对应的模块才能使用。常量，顾名思义它是不可变的，很适合用作静态配置：
    const MAX_ID: usize = usize::MAX;
    println!("用户ID允许的最大值是{}", MAX_ID);

    // 常量与普通变量的区别
    // 1. 关键字是const而不是let
    // 2. 定义常量必须指明类型（如 i32）不能省略
    // 3. 定义常量时变量的命名规则一般是全部大写
    // 4. 常量可以在任意作用域进行定义，其生命周期贯穿整个程序的生命周期。编译时编译器会尽可能将其内联到代码中，所以在不同地方对同一常量的引用并不能保证引用到相同的内存地址
    // 5. 常量的赋值只能是常量表达式/数学表达式，也就是说必须是在编译期就能计算出的值，如果需要在运行时才能得出结果的值比如函数，则不能赋值给常量表达式
    // 6. 对于变量出现重复的定义(绑定)会发生变量遮盖，后面定义的变量会遮住前面定义的变量，常量则不允许出现重复的定义

    // 静态变量
    // 静态变量允许声明一个全局的变量，常用于全局数据统计，例如我们希望用一个变量来统计程序当前的总请求数：
    static mut REQUEST_RECV: usize = 0;
    unsafe {
        REQUEST_RECV += 1;
        assert_eq!(REQUEST_RECV, 1);
    }

    // Rust 要求必须使用unsafe语句块才能访问和修改static变量，因为这种使用方式往往并不安全，其实编译器是对的，当在多线程中同时去修改时，会不可避免的遇到脏数据。
    // 只有在同一线程内或者不在乎数据的准确性时，才应该使用全局静态变量。
    // 和常量相同，定义静态变量的时候必须赋值为在编译期就可以计算出的值(常量表达式/数学表达式)，不能是运行时才能计算出的值(如函数)

    // 静态变量和常量的区别
    // 1. 静态变量不会被内联，在整个程序中，静态变量只有一个实例，所有的引用都会指向同一个地址
    // 2. 存储在静态变量中的值必须要实现 Sync trait

    // 原子类型
    // 想要全局计数器、状态控制等功能，又想要线程安全的实现，原子类型是非常好的办法。
    // static REQUEST_RECV: AtomicUsize = AtomicUsize::new(0);
    // for _ in 0..100 {
    //     REQUEST_RECV.fetch_add(1, Ordering::Relaxed);
    // }
    // println!("当前用户请求数{:?}", REQUEST_RECV);

    // 示例：全局 ID 生成器
    struct Factory {
        factory_id: usize,
    }

    static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
    const MAX_IDs: usize = usize::MAX / 2;

    fn generate_id() -> usize {
        // 检查两次溢出，否则直接加一可能导致溢出
        let current_val = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
        if current_val > MAX_IDs {
            panic!("Factory ids overflowed");
        }
        let next_id = GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        if next_id > MAX_IDs {
            panic!("Factory ids overflowed");
        }
        next_id
    }

    impl Factory {
        fn new() -> Self {
            Self {
                factory_id: generate_id(),
            }
        }
    }

    #[derive(Debug)]
    struct Config {
        a: String,
        b: String,
    }
    static mut CONFIG: Option<&mut Config> = None;

    let c = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });

    unsafe {
        // 将`c`从内存中泄漏，变成`'static`生命周期
        CONFIG = Some(Box::leak(c));
        println!("{:?}", CONFIG);
    }

    // 简单来说，全局变量可以分为两种：
    // 1. 编译期初始化的全局变量，const创建常量，static创建静态变量，Atomic创建原子类型
    // 2. 运行期初始化的全局变量，lazy_static用于懒初始化，Box::leak利用内存泄漏将一个变量的生命周期变为'static
}
