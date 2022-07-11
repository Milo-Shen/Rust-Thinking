// 引入 #![allow(unused_variables)] 属性标记，该标记会告诉编译器忽略未使用的变量，不要抛出 warning 警告
#![allow(unused_variables)]
type File = String;

fn open(f: &mut File) -> bool {
    true
}
fn close(f: &mut File) -> bool {
    true
}

// read 函数也非常有趣，它返回一个 ! 类型，这个表明该函数是一个发散函数，不会返回任何值，包括 ()
#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    // unimplemented!() 告诉编译器该函数尚未实现，
    // unimplemented!() 标记通常意味着我们期望快速完成主要代码，回头再通过搜索这些标记来完成次要代码，类似的标记还有 todo!()，当代码执行到这种未实现的地方时，程序会直接报错。
    unimplemented!()
}

pub fn complex_types() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    // 因为 unimplemented 和 ! 的原因，这行会提示 unreachable code
    // read(&mut f1, &mut vec![]);
    close(&mut f1);
}
