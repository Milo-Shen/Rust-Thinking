use std::cell::Cell;
pub fn cell_refcell() {
    // Cell 和 RefCell
    // Rust 的编译器之严格，可以说是举世无双。特别是在所有权方面，Rust 通过严格的规则来保证所有权和借用的正确性，最终为程序的安全保驾护航。
    // 但是严格是一把双刃剑，带来安全提升的同时，损失了灵活性，有时甚至会让用户痛苦不堪、怨声载道。因此 Rust 提供了 Cell 和 RefCell 用于内部可变性，简而言之，可以在拥有不可变引用的同时修改目标数据，对于正常的代码实现来说，这个是不可能做到的（要么一个可变借用，要么多个不可变借用）。
    // 内部可变性的实现是因为 Rust 使用了 unsafe 来做到这一点，但是对于使用者来说，这些都是透明的，因为这些不安全代码都被封装到了安全的 API 中

    // Cell
    // Cell 和 RefCell 在功能上没有区别，区别在于 Cell<T> 适用于 T 实现 Copy 的情况：
    let c = Cell::new("asdf");
    let one = c.get();
    c.set("qwer1s");
    let two = c.get();
    println!("{},{}", one, two);

    // 以上代码展示了 Cell 的基本用法，有几点值得注意：
    // 1. "asdf" 是 &str 类型，它实现了 Copy 特征
    // 2. c.get 用来取值，c.set 用来设置新值

    // 取到值保存在 one 变量后，还能同时进行修改，这个违背了 Rust 的借用规则，但是由于 Cell 的存在，我们很优雅地做到了这一点，但是如果你尝试在 Cell 中存放String：
    let c = Cell::new(String::from("asdf"));
    // 编译器会立刻报错，因为 String 没有实现 Copy 特征：
    // let one = c.get();
}
