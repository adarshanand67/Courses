/// Demonstrates the use of iterators in Rust by filtering even numbers from a vector.
pub fn iterators_example() {
    let numbers = vec![1, 2, 3, 4, 5];
    let even_numbers: Vec<i32> = numbers.into_iter().filter(|&x| x % 2 == 0).collect();
    println!("Even numbers: {:?}", even_numbers);
}
