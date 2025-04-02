pub fn errors_and_results() -> Result<(), String> {
    let result: Result<i32, &str> = Ok(10);
    match result {
        Ok(value) => println!("Value: {}", value),
        Err(err) => println!("Error: {}", err),
    }
    Ok(())
}

pub fn option_example() {
    let some_value: Option<i32> = Some(42);
    let no_value: Option<i32> = None;

    if let Some(value) = some_value {
        println!("Some value: {}", value);
    }

    if no_value.is_none() {
        println!("No value present");
    }
}
