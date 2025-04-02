/// Demonstrates error handling in Rust using the `Result` type.
pub fn errors_and_results() -> Result<(), String> {
    let result: Result<i32, &str> = Ok(10);
    match result {
        Ok(value) => println!("Value: {}", value),
        Err(err) => println!("Error: {}", err),
    }
    Ok(())
}