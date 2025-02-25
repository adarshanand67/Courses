#![allow(unused)]
use std::fs;
use std::io::Error; // file system module

fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n");
    let mut errors = Vec::new();
    for line in split_text {
        if line.starts_with("ERROR") {
            errors.push(line.to_string());
        }
    }
    errors
}

// basics of error handling
fn main() -> Result<(), Error> {
    // Type 1
    // match fs::read_to_string("logs.txt") {
    //     Ok(content) => {
    //         let errors = extract_errors(content.as_str());

    //         match fs::write("errors.txt", errors.join("\n")) {
    //             Ok(_) => println!("Errors written to file"),
    //             Err(e) => println!("Error writing file: {}", e),
    //         }
    //     }
    //     Err(e) => println!("Error reading file: {}", e),
    // }

    // Type 2
    // let text = fs::read_to_string("logs.txt").expect("Error reading file");
    // let errors = extract_errors(text.as_str());
    // fs::write("errors.txt", errors.join("\n")).expect("Error writing file");

    let text = fs::read_to_string("logs.txt")?;
    println!("{}", text.len());
    let errors = extract_errors(text.as_str());
    fs::write("errors.txt", errors.join("\n"))?;

    Ok(())
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err(String::from("Cannot divide by zero"));
    }
    Ok(a / b)
}
