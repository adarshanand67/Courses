//! This is the entry point of the application. It demonstrates various Rust concepts
//! by calling functions from different modules.

mod modules;

use modules::generics_and_traits::Printable;

/// The main function calls examples from different modules to demonstrate Rust features.
fn main() {
    modules::foundations::foundations();
    modules::core_concepts::core_concepts();
    modules::ownership_and_borrowing::ownership_and_borrowing();

    let longer = modules::lifetimes::lifetimes_example("short", "longer");
    println!("Longer string: {}", longer);

    modules::enums::enums_example();

    let sum = modules::math::add(5, 10);
    println!("Sum: {}", sum);

    if let Err(err) = modules::errors_and_results::errors_and_results() {
        println!("Error: {}", err);
    }

    modules::iterators::iterators_example();

    let point = modules::generics_and_traits::Point::new(3, 4);
    point.print();

    modules::structs::structs_example();
    modules::traits::traits_example();
    modules::macros::macros_example();
    modules::concurrency::concurrency_example();
}
