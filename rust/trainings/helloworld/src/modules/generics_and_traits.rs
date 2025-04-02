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
