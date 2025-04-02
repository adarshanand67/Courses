pub struct Holder<'a> {
    pub value: &'a str,
}

pub fn lifetimes_example<'a>(x: &'a str, y: &'a str) -> &'a str {
    let result: &'a str = if x.len() > y.len() { x } else { y };
    result
}

pub fn struct_with_lifetime_example() {
    let string = String::from("hello");
    let holder = Holder { value: &string };
    println!("Holder value: {}", holder.value);
}
