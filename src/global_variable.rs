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
}
