mod basic_types;
mod char_bool_unit;
mod circular_reference_and_self_reference;
mod closure;
mod complex_types;
mod deep_lifecycle;
mod deep_trait;
mod destructuring_assignment;
mod enumerations_and_integers;
mod error_handling;
mod generics;
mod global_variable;
mod hash_map;
mod learn_box;
mod learn_deref;
mod learn_drop;
mod learn_iterator;
mod learn_string;
mod learn_struct;
mod learn_trait;
mod learn_vec;
mod lifecycle;
mod lifecycle_exp_1;
mod lifecycle_exp_2;
mod method;
mod newtype_and_type_aliases;
mod notes_and_documentation;
mod option;
mod pattern_matching_1;
mod pattern_matching_2;
mod pattern_matching_3;
mod pattern_matching_4;
mod process_control;
mod rc_arc;
mod refcell;
mod reference;
mod return_values_and_error_handling;
mod rust_common_pitfalls;
mod self_referential_struct;
mod sized_dst;
mod statements_expressions;
mod static_lifecycle;
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

    println!("learn deep life cycle");
    deep_lifecycle::deep_lifecycle();

    println!("learn deep life cycle example 1");
    lifecycle_exp_1::lifecycle_exp_1();

    println!("learn deep life cycle example 2");
    lifecycle_exp_2::lifecycle_exp_2();

    println!("learn static life cycle");
    static_lifecycle::static_lifecycle();

    println!("learn closure");
    closure::closure();

    println!("learn iterator");
    learn_iterator::iterator();

    println!("learn newtype_and_type_aliases");
    newtype_and_type_aliases::newtype_and_type_aliases();

    println!("learn sized and dst");
    sized_dst::sized_dst();

    println!("learn enumerations_and_integers");
    enumerations_and_integers::enumerations_and_integers();

    println!("learn box");
    learn_box::learn_box();

    println!("learn deref");
    learn_deref::learn_deref();

    println!("learn drop");
    learn_drop::learn_drop();

    println!("learn rc arc");
    rc_arc::Rc_Arc();

    println!("learn refcell");
    refcell::cell_refcell();

    println!("learn circular reference and self-reference");
    circular_reference_and_self_reference::circular_reference_and_self_reference();

    println!("learn self_referential_struct");
    self_referential_struct::self_referential_struct();

    println!("learn global variable");
    global_variable::global_variable();

    println!("learn error handling");
    error_handling::error_handling();

    println!("learn rust common pitfalls");
    rust_common_pitfalls::rust_common_pitfalls();
}
