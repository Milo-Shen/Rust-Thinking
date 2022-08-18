pub fn unsafe_rust() {
    // 虽然在本章之前，我们学到的代码都是在编译期就得到了 Rust 的安全保障，但是在其内心深处也隐藏了一些阴暗面，
    // 在这些阴暗面里，内存安全就存在一些变数了：当不娴熟的开发者接触到这些阴暗面，就可能写出不安全的代码，因此我们称这种代码为 unsafe 代码块。
    let mut num = 5;

    let r1 = &num as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
    }

    // 上面代码中, r1 是一个裸指针(raw pointer)，由于它具有破坏 Rust 内存安全的潜力，因此只能在 unsafe 代码块中使用，如果你去掉 unsafe {}，编译器会立刻报错。
    // 言归正传， unsafe 能赋予我们 5 种超能力，这些能力在安全的 Rust 代码中是无法获取的：
    // 1. 解引用裸指针，就如上例所示
    // 2. 调用一个 unsafe 或外部的函数
    // 3. 访问或修改一个可变的静态变量
    // 4. 实现一个 unsafe 特征
    // 5. 访问 union 中的字段

    // 解引用裸指针
    // 裸指针(raw pointer，又称原生指针) 在功能上跟引用类似，同时它也需要显式地注明可变性。但是又和引用有所不同，裸指针长这样: *const T 和 *mut T，它们分别代表了不可变和可变。
    // 大家在之前学过 * 操作符，知道它可以用于解引用，但是在裸指针 *const T 中，这里的 * 只是类型名称的一部分，并没有解引用的含义。
    // 至此，我们已经学过三种类似指针的概念：引用、智能指针和裸指针。与前两者不同，裸指针：
    // 1. 可以绕过 Rust 的借用规则，可以同时拥有一个数据的可变、不可变指针，甚至还能拥有多个可变的指针
    // 2. 并不能保证指向合法的内存
    // 3. 可以是 null
    // 4. 没有实现任何自动的回收 (drop)

    // 总之，裸指针跟 C 指针是非常像的，使用它需要以牺牲安全性为前提，但我们获得了更好的性能，也可以跟其它语言或硬件打交道。

    // 基于引用创建裸指针
    // 下面的代码基于值的引用同时创建了可变和不可变的裸指针：
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 细心的同学可能会发现，在这段代码中并没有 unsafe 的身影，原因在于：创建裸指针是安全的行为，而解引用裸指针才是不安全的行为:
    let mut num = 5;
    unsafe {
        println!("r1 is: {}", *r2);
    }

    // 基于内存地址创建裸指针
    // 在上面例子中，我们基于现有的引用来创建裸指针，这种行为是很安全的。但是接下来的方式就不安全了：
    let address = 0x012345usize;
    let r = address as *const i32;

    // 这里基于一个内存地址来创建裸指针，可以想像，这种行为是相当危险的。
    // 试图使用任意的内存地址往往是一种未定义的行为(undefined behavior)，因为该内存地址有可能存在值，也有可能没有，就算有值，也大概率不是你需要的值。
    // 同时编译器也有可能会优化这段代码，会造成没有任何内存访问发生，甚至程序还可能发生段错误(segmentation fault)。总之，你几乎没有好的理由像上面这样实现代码，虽然它是可行的。

    // 使用 * 解引用
    let a = 1;
    let b: *const i32 = &a as *const i32;
    let c: *const i32 = &a;
    unsafe {
        println!("{}", *c);
    }

    // 基于智能指针创建裸指针
    let a: Box<i32> = Box::new(10);
    // 需要先解引用a
    let b: *const i32 = &*a;
    // 使用 into_raw 来创建
    let c: *const i32 = Box::into_raw(a);
}
