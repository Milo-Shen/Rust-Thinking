mod basic_types;
mod char_bool_unit;
mod complex_types;
mod deep_trait;
mod destructuring_assignment;
mod generics;
mod hash_map;
mod learn_string;
mod learn_struct;
mod learn_trait;
mod learn_vec;
mod lifecycle;
mod method;
mod notes_and_documentation;
mod option;
mod pattern_matching_1;
mod pattern_matching_2;
mod pattern_matching_3;
mod pattern_matching_4;
mod process_control;
mod reference;
mod return_values_and_error_handling;
mod statements_expressions;
mod trait_obj;
mod tuple;
mod type_conversion;

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

    println!("learn options");
    option::option();

    println!("learn pattern matching");
    pattern_matching_1::pattern_matching();

    println!("learn pattern matching 2");
    pattern_matching_2::pattern_matching_2();

    println!("learn pattern matching 3");
    pattern_matching_3::pattern_matching_3();

    println!("learn pattern matching 4");
    pattern_matching_4::pattern_matching_4();

    println!("learn method");
    method::method();

    println!("learn generics");
    generics::generics();

    println!("learn trait");
    learn_trait::learn_trait();
    trait_obj::trait_obj();

    println!("learn deep trait");
    deep_trait::deep_trait();

    println!("learn vec");
    learn_vec::learn_vec();

    println!("learn hashmap");
    hash_map::hash_map();

    println!("learn type conversion");
    type_conversion::type_conversion();

    println!("learn return_values_and_error_handling");
    return_values_and_error_handling::return_values_and_error_handling();

    println!("learn Notes and documentation");
    notes_and_documentation::notes_and_documentation();

    println!("learn Life Cycle");
    lifecycle::lifecycle();
}
