/// Represents directions for demonstration purposes.
enum Direction {
    Up,
    _Down,
    _Left,
    _Right,
}

/// Represents different types of data for demonstration purposes.
#[derive(Debug)]
enum E {
    Number(i32),
    FloatingPoint(f64),
    _Tuple(char, bool),
    _Word(String),
}

/// Processes an enum value and prints its details.
fn process_enum(e: E) {
    match e {
        E::Number(n) => println!("Number: {}", n),
        E::FloatingPoint(f) => println!("Floating point: {}", f),
        E::_Tuple(c, b) => println!("Tuple: ({}, {})", c, b),
        E::_Word(s) => println!("Word: {}", s),
    }
}

/// Demonstrates the use of enums and pattern matching in Rust.
pub fn enums_example() {
    let direction: Direction = Direction::Up;
    match direction {
        Direction::Up => println!("Going up"),
        Direction::_Down => println!("Going down"),
        Direction::_Left => println!("Going left"),
        Direction::_Right => println!("Going right"),
    }
    let e: E = E::Number(-5);
    process_enum(e);

    let e: E = E::FloatingPoint(3.14);
    process_enum(e);
}
