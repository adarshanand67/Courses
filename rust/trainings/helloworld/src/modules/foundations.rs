pub fn foundations() {
    println!("Why Rust? Memory safe, thread safe, low-level systems programming language.");
}

pub fn ownership_example() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s2: {}", s2);
}
