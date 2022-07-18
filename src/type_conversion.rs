use std::rc::Rc;

pub fn type_conversion() {
    // 每个类型能表达的数据范围不同，如果把范围较大的类型转换成较小的类型，会造成错误，因此我们需要把范围较小的类型转换成较大的类型，来避免这些问题的发生。
    let a = i8::MAX;
    println!("{}", a);

    let a = 3.1 as i8;
    let b = 100_i8 as i32;
    let c = 'a' as u8;
    println!("{},{},{}", a, b, c);

    // 内存地址转换为指针
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address = p1 as usize; // 将p1内存地址转换为一个整数
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()，i32类型占用4个字节，因此将内存地址 + 4
    let p2 = second_address as *mut i32; // 访问该地址指向的下一个整数p2
    unsafe {
        *p2 += 1;
    }
    assert_eq!(values[1], 3);

    // 强制类型转换的边角知识
    // 转换不具有传递性 就算 e as U1 as U2 是合法的，也不能说明 e as U2 是合法的（e 不能直接转换成 U2）。

    // TryInto 转换
    // 在一些场景中，使用 as 关键字会有比较大的限制。如果你想要在类型转换上拥有完全的控制而不依赖内置的转换，例如处理转换错误，那么可以使用 TryInto:

    let a: u8 = 10;
    let b: u16 = 1;
    let b_: u8 = b.try_into().unwrap();
    if a < b_ {
        println!("Ten is less than one hundred.");
    }

    // try_into 转换会捕获大类型向小类型转换时导致的溢出错误：
    let b: i16 = 1500;

    let b_: u8 = match b.try_into() {
        Ok(b1) => b1,
        Err(e) => {
            println!("{:?}", e.to_string());
            0
        }
    };

    // 通用类型转换
    // 虽然 as 和 TryInto 很强大，但是只能应用在数值类型上，可是 Rust 有如此多的类型，想要为这些类型实现转换，我们需要另谋出路，先来看看在一个笨办法，将一个结构体转换为另外一个结构体：
    struct Foo {
        x: u32,
        y: u16,
    }

    struct Bar {
        a: u32,
        b: u16,
    }

    fn reinterpret(foo: Foo) -> Bar {
        let Foo { x, y } = foo;
        Bar { a: x, b: y }
    }

    // 强制类型转换
    // 在某些情况下，类型是可以进行隐式强制转换的，虽然这些转换弱化了 Rust 的类型系统，但是它们的存在是为了让 Rust 在大多数场景可以工作(说白了，帮助用户省事)，而不是报各种类型上的编译错误。

    trait Trait {}
    fn foo<X: Trait>(t: X) {}
    impl<'a> Trait for &'a i32 {}
    let t = &mut 0;
    // foo(t);
    // &i32 实现了特征 Trait， &mut i32 可以转换为 &i32，但是 &mut i32 依然无法作为 Trait 来使用。

    // 点操作符
    // 方法调用的点操作符看起来简单，实际上非常不简单，它在调用时，会发生很多魔法般的类型转换，例如：自动引用、自动解引用，强制类型转换直到类型能匹配等。
    // 假设有一个方法 foo，它有一个接收器(接收器就是 self、&self、&mut self 参数)。如果调用 value.foo()，编译器在调用 foo 之前，需要决定到底使用哪个 Self 类型来调用。现在假设 value 拥有类型 T。

    // 再进一步，我们使用完全限定语法来进行准确的函数调用:
    // 1. 首先，编译器检查它是否可以直接调用 T::foo(value)，称之为值方法调用
    // 2. 如果上一步调用无法完成(例如方法类型错误或者特征没有针对 Self 进行实现，上文提到过特征不能进行强制转换)，那么编译器会尝试增加自动引用，例如会尝试以下调用： <&T>::foo(value) 和 <&mut T>::foo(value)，称之为引用方法调用
    // 3. 若上面两个方法依然不工作，编译器会试着解引用 T ，然后再进行尝试。这里使用了 Deref 特征 —— 若 T: Deref<Target = U> (T 可以被解引用为 U)，那么编译器会使用 U 类型进行尝试，称之为解引用方法调用
    // 4. 若 T 不能被解引用，且 T 是一个定长类型(在编译器类型长度是已知的)，那么编译器也会尝试将 T 从定长类型转为不定长类型，例如将 [i32; 2] 转为 [i32]
    // 5. 若还是不行，那...没有那了，最后编译器大喊一声：汝欺我甚，不干了！

    let array: Rc<Box<[i32; 3]>> = Rc::new(Box::new([1, 2, 3]));
    let first_entry = array[0];
    println!("first_entry = {}", first_entry);

    // array 数组的底层数据隐藏在了重重封锁之后，那么编译器如何使用 array[0] 这种数组原生访问语法通过重重封锁，准确的访问到数组中的第一个元素？
    // 1. 首先， array[0] 只是Index特征的语法糖：编译器会将 array[0] 转换为 array.index(0) 调用，当然在调用之前，编译器会先检查 array 是否实现了 Index 特征。
    // 2. 接着，编译器检查 Rc<Box<[T; 3]>> 是否有否实现 Index 特征，结果是否，不仅如此，&Rc<Box<[T; 3]>> 与 &mut Rc<Box<[T; 3]>> 也没有实现。
    // 3. 上面的都不能工作，编译器开始对 Rc<Box<[T; 3]>> 进行解引用，把它转变成 Box<[T; 3]>
    // 4. 此时继续对 Box<[T; 3]> 进行上面的操作 ：Box<[T; 3]>， &Box<[T; 3]>，和 &mut Box<[T; 3]> 都没有实现 Index 特征，所以编译器开始对 Box<[T; 3]> 进行解引用，然后我们得到了 [T; 3]
    // 5. [T; 3] 以及它的各种引用都没有实现 Index 索引(是不是很反直觉:D，在直觉中，数组都可以通过索引访问，实际上只有数组切片才可以!)，它也不能再进行解引用，因此编译器只能祭出最后的大杀器：将定长转为不定长，因此 [T; 3] 被转换成 [T]，也就是数组切片，它实现了 Index 特征，因此最终我们可以通过 index 方法访问到对应的元素。
}
