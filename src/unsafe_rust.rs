pub fn unsafe_rust() {
    // 虽然在本章之前，我们学到的代码都是在编译期就得到了 Rust 的安全保障，但是在其内心深处也隐藏了一些阴暗面，
    // 在这些阴暗面里，内存安全就存在一些变数了：当不娴熟的开发者接触到这些阴暗面，就可能写出不安全的代码，因此我们称这种代码为 unsafe 代码块。
    let mut num = 5;

    let r1 = &num as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
    }
}
