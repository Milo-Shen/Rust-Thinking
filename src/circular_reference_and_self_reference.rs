use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub fn circular_reference_and_self_reference() {
    // 循环引用与自引用
    // 实现一个链表是学习各大编程语言的常用技巧，但是在 Rust 中实现链表意味着····Hell，是的，你没看错，Welcome to hell。
    // 链表在 Rust 中之所以这么难，完全是因为循环引用和自引用的问题引起的，这两个问题可以说综合了 Rust 的很多难点，难出了新高度，因此本书专门开辟一章，分为上下两篇，试图彻底解决这两个老大难。
    // 本章难度较高，但是非常值得深入阅读，它会让你对 Rust 的理解上升到一个新的境界。

    // Rust 的安全性是众所周知的，但是不代表它不会内存泄漏。一个典型的例子就是同时使用 Rc<T> 和 RefCell<T> 创建循环引用，最终这些引用的计数都无法被归零，因此 Rc<T> 拥有的值也不会被释放清理。
    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    use List::{Cons, Nil};

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }

    // 这里我们创建一个有些复杂的枚举类型 List，这个类型很有意思，它的每个值都指向了另一个 List，此外，得益于 Rc 的使用还允许多个值指向一个 List
    // 如上图所示，每个矩形框节点都是一个 List 类型，它们或者是拥有值且指向另一个 List 的 Cons，或者是一个没有值的终结点 Nil。同时，由于 RefCell 的使用，每个 List 所指向的 List 还能够被修改。
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a的初始化rc计数 = {}", Rc::strong_count(&a));
    println!("a指向的节点 = {:?}", a.tail());

    // 创建`b`到`a`的引用
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("在b创建后，a的rc计数 = {}", Rc::strong_count(&a));
    println!("b的初始化rc计数 = {}", Rc::strong_count(&b));
    println!("b指向的节点 = {:?}", b.tail());

    // 利用RefCell的可变性，创建了`a`到`b`的引用
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("在更改a后，b的rc计数 = {}", Rc::strong_count(&b));
    println!("在更改a后，a的rc计数 = {}", Rc::strong_count(&a));

    // 下面一行println!将导致循环引用
    // 我们可怜的8MB大小的main线程栈空间将被它冲垮，最终造成栈溢出
    // println!("a next item = {:?}", a.tail());

    // 在 main 函数结束前，a 和 b 的引用计数均是 2，随后 b 触发 Drop，此时引用计数会变为 1，并不会归 0，因此 b 所指向内存不会被释放，同理可得 a 指向的内存也不会被释放，最终发生了内存泄漏。
    // 通过 a.tail 的调用，Rust 试图打印出 a -> b -> a ··· 的所有内容，但是在不懈的努力后，main 线程终于不堪重负，发生了栈溢出。
    // 以上的代码可能并不会造成什么大的问题，但是在一个更加复杂的程序中，类似的问题可能会造成你的程序不断地分配内存、泄漏内存，最终程序会不幸OOM，当然这其中的 CPU 损耗也不可小觑。
    // 总之，创建循环引用并不简单，但是也并不是完全遇不到，当你使用 RefCell<Rc<T>> 或者类似的类型嵌套组合（具备内部可变性和引用计数）时，就要打起万分精神，前面可能是深渊！
    // 那么问题来了？ 如果我们确实需要实现上面的功能，该怎么办？答案是使用 Weak。

    // Weak
    // Weak 非常类似于 Rc，但是与 Rc 持有所有权不同，Weak 不持有所有权，它仅仅保存一份指向数据的弱引用：如果你想要访问数据，需要通过 Weak 指针的 upgrade 方法实现，该方法返回一个类型为 Option<Rc<T>> 的值。
    // 看到这个返回，相信大家就懂了：何为弱引用？就是不保证引用关系依然存在，如果不存在，就返回一个 None！
    // 因为 Weak 引用不计入所有权，因此它无法阻止所引用的内存值被释放掉，而且 Weak 本身不对值的存在性做任何担保，引用的值还存在就返回 Some，不存在就返回 None。

    // Weak 与 Rc 对比
    // 我们来将 Weak 与 Rc 进行以下简单对比：

    // Weak	                                         Rc
    // 不计数	                                      引用计数
    // 不拥有所有权	                                   有值的所有权
    // 不阻止值被释放(drop)	                            所有权计数归零，才能 drop
    // 引用的值存在返回 Some，不存在返回 None	          引用的值必定存在
    // 通过 upgrade 取到 Option<Rc<T>>，然后再取值	     通过 Deref 自动解引用，取值无需任何操作

    // 通过这个对比，可以非常清晰的看出 Weak 为何这么弱，而这种弱恰恰非常适合我们实现以下的场景：
    // 1. 持有一个 Rc 对象的临时引用，并且不在乎引用的值是否依然存在
    // 2. 阻止 Rc 导致的循环引用，因为 Rc 的所有权机制，会导致多个 Rc 都无法计数归零

    // 使用方式简单总结下：对于父子引用关系，可以让父节点通过 Rc 来引用子节点，然后让子节点通过 Weak 来引用父节点。

    // Weak 总结
    // 因为 Weak 本身并不是很好理解，因此我们再来帮大家梳理总结下，然后再通过一个例子，来彻底掌握。
    // Weak 通过 use std::rc::Weak 来引入，它具有以下特点:
    // 1. 可访问，但没有所有权，不增加引用计数，因此不会影响被引用值的释放回收
    // 2. 可由 Rc<T> 调用 downgrade 方法转换成 Weak<T>
    // 3. Weak<T> 可使用 upgrade 方法转换成 Option<Rc<T>>，如果资源已经被释放，则 Option 的值是 None
    // 4. 常用于解决循环引用的问题
    // 创建Rc，持有一个值5
    let five = Rc::new(5);

    // 通过Rc，创建一个Weak指针
    let weak_five = Rc::downgrade(&five);

    // Weak引用的资源依然存在，取到值5
    let strong_five = weak_five.upgrade();
    assert_eq!(*strong_five.unwrap(), 5);

    // 手动释放资源`five`
    drop(five);

    // Weak引用的资源已不存在，因此返回None
    let strong_five = weak_five.upgrade();
    assert_eq!(strong_five, None);
}
