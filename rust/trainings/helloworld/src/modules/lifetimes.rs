/// Demonstrates lifetime annotations in Rust by returning the longer of two string slices.
///
/// # Arguments
/// * `x` - A string slice.
/// * `y` - Another string slice.
///
/// # Returns
/// The longer of the two string slices.
pub fn lifetimes_example<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
