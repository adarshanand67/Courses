//! This is the entry point of the application. It demonstrates various Rust concepts
//! by calling functions from different modules.

mod modules;

use modules::generics_and_traits::Printable;

fn main() {
    modules::foundations::foundations();
    modules::foundations::ownership_example();

    modules::core_concepts::core_concepts();
    modules::ownership_and_borrowing::ownership_and_borrowing();

    let longer = modules::lifetimes::lifetimes_example("short", "longer");
    println!("Longer string: {}", longer);
    modules::lifetimes::struct_with_lifetime_example();

    modules::enums::enums_example();

    let sum = modules::math::add(5, 10);
    println!("Sum: {}", sum);
    let difference = modules::math::subtract(10, 5);
    println!("Difference: {}", difference);

    if let Err(err) = modules::errors_and_results::errors_and_results() {
        println!("Error: {}", err);
    }
    modules::errors_and_results::option_example();

    modules::iterators::iterators_example();

    let point = modules::generics_and_traits::Point::new(3, 4);
    point.print();
    let larger = modules::generics_and_traits::max(10, 20);
    println!("Larger number: {}", larger);
    modules::generics_and_traits::generics_example();

    modules::structs::structs_example();
    modules::traits::traits_example();
    modules::macros::macros_example();
    modules::concurrency::concurrency_example();

    modules::advanced_traits::advanced_traits_example();
}
