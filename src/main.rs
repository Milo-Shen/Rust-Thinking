mod basic_types;
mod char_bool_unit;
mod complex_types;
mod destructuring_assignment;
mod learn_string;
mod learn_struct;
mod process_control;
mod reference;
mod statements_expressions;
mod tuple;

fn main() {
    println!("learn destructuring_assignment");
    destructuring_assignment::destructuring_assignment();

    println!("learn basic types");
    basic_types::basic_types();

    println!("learn char bool unit");
    char_bool_unit::char_bool_unit();

    println!("learn statements and expressions");
    statements_expressions::statements_expressions();

    println!("learn references");
    reference::reference();

    println!("learn complex types");
    complex_types::complex_types();

    println!("learn string");
    learn_string::learning_string();

    println!("learn tuple");
    tuple::tuple();

    println!("learn struct");
    learn_struct::learn_struct();

    println!("learn process control");
    process_control::process_control();
}
