/// Demonstrates advanced trait usage in Rust.

pub trait AdvancedTrait {
    fn advanced_method(&self) -> String;
}

pub struct ExampleStruct {
    pub value: i32,
}

impl AdvancedTrait for ExampleStruct {
    fn advanced_method(&self) -> String {
        format!("Advanced value: {}", self.value)
    }
}

/// Demonstrates the use of advanced traits.
pub fn advanced_traits_example() {
    let example = ExampleStruct { value: 42 };
    println!("{}", example.advanced_method());
}
