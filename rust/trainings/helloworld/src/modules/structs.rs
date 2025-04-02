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

/// A tuple struct representing a 3D point.
pub struct Point3D(pub i32, pub i32, pub i32);

/// Demonstrates the use of structs in Rust.
pub fn structs_example() {
    let rect: Rectangle = Rectangle {
        width: 10,
        height: 20,
    };
    println!("Rectangle area: {}", rect.area());

    let point = Point3D(1, 2, 3);
    println!("Point3D({}, {}, {})", point.0, point.1, point.2);
}
