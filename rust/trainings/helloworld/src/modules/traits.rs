/// A trait for types that can provide a description of themselves.
pub trait Describable {
    /// Returns a description of the implementing type.
    fn describe(&self) -> String;
}

impl Describable for super::structs::Rectangle {
    fn describe(&self) -> String {
        format!(
            "Rectangle with width {} and height {}",
            self.width, self.height
        )
    }
}

/// Demonstrates the use of traits in Rust.
pub fn traits_example() {
    let rect: super::structs::Rectangle = super::structs::Rectangle {
        width: 15,
        height: 25,
    };
    println!("{}", rect.describe());
}
