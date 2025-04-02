/// A struct representing a rectangle with width and height.
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    /// Calculates the area of the rectangle.
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

/// Demonstrates the use of structs in Rust.
pub fn structs_example() {
    let rect: Rectangle = Rectangle {
        width: 10,
        height: 20,
    };
    println!("Rectangle area: {}", rect.area());
}
