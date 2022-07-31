use std::rc::Rc;

pub fn Rc_Arc() {
    // Rust 所有权机制要求一个值只能有一个所有者，在大多数情况下，都没有问题，但是考虑以下情况：
    // 1. 在图数据结构中，多个边可能会拥有同一个节点，该节点直到没有边指向它时，才应该被释放清理
    // 2. 在多线程中，多个线程可能会持有同一个数据，但是你受限于 Rust 的安全机制，无法同时获取该数据的可变引用

    // 以上场景不是很常见，但是一旦遇到，就非常棘手，为了解决此类问题，Rust 在所有权机制之外又引入了额外的措施来简化相应的实现：通过引用计数的方式，允许一个数据资源在同一时刻拥有多个所有者。
    // 这种实现机制就是 Rc 和 Arc，前者适用于单线程，后者适用于多线程。由于二者大部分情况下都相同，因此本章将以 Rc 作为讲解主体，对于 Arc 的不同之处，另外进行单独讲解。

    // Rc<T>
    // 引用计数(reference counting)，顾名思义，通过记录一个数据被引用的次数来确定该数据是否正在被使用。当引用次数归零时，就代表该数据不再被使用，因此可以被清理释放。

    // 而 Rc 正是引用计数的英文缩写。当我们希望在堆上分配一个对象供程序的多个部分使用且无法确定哪个部分最后一个结束时，就可以使用 Rc 成为数据值的所有者，例如之前提到的多线程场景就非常适合。
    // s 在这里被转移给 a
    let s = String::from("hello, world");
    let a = Box::new(s);
    // 报错！此处继续尝试将 s 转移给 b
    // let b = Box::new(s);
    let a = Rc::new(String::from("hello, world"));
    let b = Rc::clone(&a);
    let c = a.clone();

    assert_eq!(3, Rc::strong_count(&a));
    assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b));

    // 以上代码我们使用 Rc::new 创建了一个新的 Rc<String> 智能指针并赋给变量 a，该指针指向底层的字符串数据。
    // 智能指针 Rc<T> 在创建时，还会将引用计数加 1，此时获取引用计数的关联函数 Rc::strong_count 返回的值将是 1。

    // Rc::clone
    // 接着，我们又使用 Rc::clone 克隆了一份智能指针 Rc<String>，并将该智能指针的引用计数增加到 2。
    // 由于 a 和 b 是同一个智能指针的两个副本，因此通过它们两个获取引用计数的结果都是 2。
    // 不要被 clone 字样所迷惑，以为所有的 clone 都是深拷贝。这里的 clone 仅仅复制了智能指针并增加了引用计数，并没有克隆底层数据，因此 a 和 b 是共享了底层的字符串 s，这种复制效率是非常高的。当然你也可以使用 a.clone() 的方式来克隆，但是从可读性角度，我们更加推荐 Rc::clone 的方式。
    // 实际上在 Rust 中，还有不少 clone 都是浅拷贝，例如迭代器的克隆。

    // 观察引用计数的变化
    // 使用关联函数 Rc::strong_count 可以获取当前引用计数的值，我们来观察下引用计数如何随着变量声明、释放而变化：
    let a = Rc::new(String::from("test ref counting"));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Rc::clone(&a);
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Rc::clone(&a);
        println!("count after creating c = {}", Rc::strong_count(&c));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // 有几点值得注意：
    // 1. 由于变量 c 在语句块内部声明，当离开语句块时它会因为超出作用域而被释放，所以引用计数会减少 1，事实上这个得益于 Rc<T> 实现了 Drop 特征
    // 2. a、b、c 三个智能指针引用计数都是同样的，并且共享底层的数据，因此打印计数时用哪个都行
    // 3. 无法看到的是：当 a、b 超出作用域后，引用计数会变成 0，最终智能指针和它指向的底层字符串都会被清理释放
}
