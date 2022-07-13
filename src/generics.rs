#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

pub fn generics() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("{:?}, {:?}", integer, float);
}
