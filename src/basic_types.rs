pub fn basic_types() {
    let guess: i32 = "42".parse().expect("Not a number!");
    let guess_a = "42".parse::<i32>().expect("Not a number!");
    println!("{}, {}", guess, guess_a);
}
