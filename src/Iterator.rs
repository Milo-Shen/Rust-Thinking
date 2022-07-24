pub fn iterator() {
    // 迭代器允许我们迭代一个连续的集合，例如数组、动态数组 Vec、HashMap 等，在此过程中，只需关心集合中的元素如何处理，而无需关心如何开始、如何结束、按照什么样的索引去访问等问题。
    let arr = [1, 2, 3];
    for v in arr {
        println!("{}", v);
    }

    // 首先，不得不说这两语法还挺像！与 JS 循环不同，Rust中没有使用索引，它把 arr 数组当成一个迭代器，直接去遍历其中的元素，从哪里开始，从哪里结束，都无需操心。因此严格来说，Rust 中的 for 循环是编译器提供的语法糖，最终还是对迭代器中的元素进行遍历。
    // 那又有同学要发问了，在 Rust 中数组是迭代器吗？因为在之前的代码中直接对数组 arr 进行了迭代，答案是 No。那既然数组不是迭代器，为啥咱可以对它的元素进行迭代呢？
    // 简而言之就是数组实现了 IntoIterator 特征，Rust 通过 for 语法糖，自动把实现了该特征的数组类型转换为迭代器（你也可以为自己的集合类型实现此特征），最终让我们可以直接对一个数组进行迭代，类似的还有：
    for i in 1..10 {
        println!("{}", i);
    }

    // 直接对数值序列进行迭代，也是很常见的使用方式。
    // IntoIterator 特征拥有一个 into_iter 方法，因此我们还可以显式的把数组转换成迭代器：
    let arr = [1, 2, 3];
    for v in arr.into_iter() {
        println!("{}", v);
    }

    // 惰性初始化
    // 在 for 循环之前，我们只是简单的创建了一个迭代器 v1_iter，此时不会发生任何迭代行为，只有在 for 循环开始后，迭代器才会开始迭代其中的元素，最后打印出来。
    // 这种惰性初始化的方式确保了创建迭代器不会有任何额外的性能损耗，其中的元素也不会被消耗，只有使用到该迭代器的时候，一切才开始。
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("{}", val);
    }

    // 因此，之前问题的答案已经很明显：for 循环通过不停调用迭代器上的 next 方法，来获取迭代器中的元素。
    let arr = [1, 2, 3];
    let mut arr_iter = arr.into_iter();
    assert_eq!(arr_iter.next(), Some(1));
    assert_eq!(arr_iter.next(), Some(2));
    assert_eq!(arr_iter.next(), Some(3));
    assert_eq!(arr_iter.next(), None);

    // 果不其然，将 arr 转换成迭代器后，通过调用其上的 next 方法，我们获取了 arr 中的元素，有两点需要注意：
    // 1. next 方法返回的是 Option 类型，当有值时返回 Some(i32)，无值时返回 None
    // 2. 遍历是按照迭代器中元素的排列顺序依次进行的，因此我们严格按照数组中元素的顺序取出了 Some(1)，Some(2)，Some(3)
    // 3. 手动迭代必须将迭代器声明为 mut 可变，因为调用 next 会改变迭代器其中的状态数据（当前遍历的位置等），而 for 循环去迭代则无需标注 mut，因为它会帮我们自动完成
    // 4. 总之，next 方法对迭代器的遍历是消耗性的，每次消耗它一个元素，最终迭代器中将没有任何元素，只能返回 None。

    // 例子：模拟实现 for 循环
    let values = vec![1, 2, 3];
    let result = match IntoIterator::into_iter(values) {
        mut iter => loop {
            match iter.next() {
                Some(x) => {
                    println!("{}", x);
                }
                None => break,
            }
        },
    };

    // IntoIterator 特征
    // 其实有一个细节，由于 Vec 动态数组实现了 IntoIterator 特征，因此可以通过 into_iter 将其转换为迭代器，那如果本身就是一个迭代器，该怎么办？实际上，迭代器自身也实现了 IntoIterator，标准库早就帮我们考虑好了：
    let values = vec![1, 2, 3];
    for v in values.into_iter().into_iter().into_iter() {
        println!("{}", v)
    }

    // into_iter, iter, iter_mut
    // 在之前的代码中，我们统一使用了 into_iter 的方式将数组转化为迭代器，除此之外，还有 iter 和 iter_mut，聪明的读者应该大概能猜到这三者的区别：
    // 1. into_iter 会夺走所有权
    // 2. iter 是借用
    // 3. iter_mut 是可变借用
}
