/// A struct that holds a reference with a lifetime annotation.
pub struct Holder<'a> {
    pub value: &'a str,
}

/// Demonstrates lifetime annotations in Rust by returning the longer of two string slices.
///
/// # Arguments
/// * `x` - A string slice.
/// * `y` - Another string slice.
///
/// # Returns
/// The longer of the two string slices.
pub fn lifetimes_example<'a>(x: &'a str, y: &'a str) -> &'a str {
    let result: &'a str = if x.len() > y.len() { x } else { y };
    result
}

/// Demonstrates the use of a struct with lifetime annotations.
pub fn struct_with_lifetime_example() {
    let string = String::from("hello");
    let holder = Holder { value: &string };
    println!("Holder value: {}", holder.value);
}
