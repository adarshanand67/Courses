/// A generic struct representing a point in 2D space.
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    /// Creates a new `Point` with the given coordinates.
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

/// A trait for types that can be printed.
pub trait Printable {
    /// Prints the details of the implementing type.
    fn print(&self);
}

impl Printable for Point<i32> {
    fn print(&self) {
        println!("Point({}, {})", self.x, self.y);
    }
}

/// A generic function that returns the larger of two values.
pub fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

/// Demonstrates the use of the generic function.
pub fn generics_example() {
    let larger = max(10, 20);
    println!("Larger number: {}", larger);

    let larger_float = max(3.14, 2.71);
    println!("Larger float: {}", larger_float);
}
