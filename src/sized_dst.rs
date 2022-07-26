pub fn sized_dst() {
    // Sized 和不定长类型 DST

    // 在 Rust 中类型有多种抽象的分类方式，例如本书之前章节的：基本类型、集合类型、复合类型等。再比如说，如果从编译器何时能获知类型大小的角度出发，可以分成两类:
    // 1. 定长类型( sized )，这些类型的大小在编译时是已知的
    // 2. 不定长类型( unsized )，与定长类型相反，它的大小只有到了程序运行时才能动态获知，这种类型又被称之为 DST

    // 动态大小类型 DST
    // 读者大大们之前学过的几乎所有类型，都是固定大小的类型，包括集合 Vec、String 和 HashMap 等，而动态大小类型刚好与之相反：编译器无法在编译期得知该类型值的大小，只有到了程序运行时，才能动态获知。对于动态类型，我们使用 DST(dynamically sized types)或者 unsized 类型来称呼它。
    // 上述的这些集合虽然底层数据可动态变化，感觉像是动态大小的类型。但是实际上，这些底层数据只是保存在堆上，在栈中还存有一个引用类型，该引用包含了集合的内存地址、元素数目、分配空间信息，通过这些信息，编译器对于该集合的实际大小了若指掌，最最重要的是：栈上的引用类型是固定大小的，因此它们依然是固定大小的类型。
    // 正因为编译器无法在编译期获知类型大小，若你试图在代码中直接使用 DST 类型，将无法通过编译。

    // 试图创建动态大小的数组
    // fn my_function(n: usize) {
    //     let array = [123; n];
    // }
    // 以上代码就会报错(错误输出的内容并不是因为 DST，但根本原因是类似的)，因为 n 在编译期无法得知，而数组类型的一个组成部分就是长度，长度变为动态的，自然类型就变成了 unsized 。

    // 切片
    // Rust 语言特性内置的 str 和 [u8] 类型都是切片，前者是字符串切片，后者是数组切片，下面我们来尝试下使用 str ：
    // let string: str = "banana";
    // let a: [u8];
    // 编译器准确的告诉了我们原因：str 字符串切片它是 DST 动态大小类型，这意味着编译器无法在编译期知道 str 类型的大小，只有到了运行期才能动态获知，这对于强类型、强安全的 Rust 语言来说是不可接受的。
    // 也就是说，我们无法直接使用 str，而对于 [u8] 也是类似的。
    // 总之，我们可以总结出一个结论：在 Rust 中，所有的切片都是动态大小类型，它们都无法直接被使用。
}
