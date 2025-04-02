//! This is the entry point of the application. It demonstrates various Rust concepts
//! by calling functions from different modules.

mod concurrency;
mod core_concepts;
mod enums;
mod errors_and_results;
mod foundations;
mod generics_and_traits;
mod iterators;
mod lifetimes;
mod macros;
mod math;
mod ownership_and_borrowing;
mod structs;
mod traits;

use generics_and_traits::Printable;

/// The main function calls examples from different modules to demonstrate Rust features.
fn main() {
    foundations::foundations();
    core_concepts::core_concepts();
    ownership_and_borrowing::ownership_and_borrowing();

    let longer = lifetimes::lifetimes_example("short", "longer");
    println!("Longer string: {}", longer);

    enums::enums_example();

    let sum = math::add(5, 10);
    println!("Sum: {}", sum);

    if let Err(err) = errors_and_results::errors_and_results() {
        println!("Error: {}", err);
    }

    iterators::iterators_example();

    let point = generics_and_traits::Point::new(3, 4);
    point.print();

    structs::structs_example();
    traits::traits_example();
    macros::macros_example();
    concurrency::concurrency_example();
}
