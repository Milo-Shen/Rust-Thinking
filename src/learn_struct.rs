#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn learn_struct() {
    // 初始化实例时，每个字段都需要进行初始化
    // 初始化时的字段顺序不需要和结构体定义时的顺序一致

    // 访问结构体字段
    // 通过 . 操作符即可访问结构体实例内部的字段值，也可以修改它们 :
    // 需要注意的是，必须要将结构体实例声明为可变的，才能修改其中的字段，Rust 不支持将某个结构体某个字段标记为可变。
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    user1.username = String::from("someusername123");
    user1.sign_in_count = 1;
    user1.active = true;
    println!("{:?}", user1);
}
