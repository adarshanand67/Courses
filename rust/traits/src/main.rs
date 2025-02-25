// Traits and generis in Rust
mod basket;
mod stack;
mod container;

use basket::Basket;
use stack::Stack;
use container::Container;

fn add_string<T: Container<String>>(container: &mut T, item: String) {
    container.put(item);
}

fn main() {
    let mut b1 = Basket::new("apple".to_string());
    let b2 = Basket::new(3.14);

    let s1 = Stack::new(vec![1, 2, 3]);
    let s2 = Stack::new(vec!["a", "b", "c"]);

    add_string(&mut b1, "banana".to_string());
    add_string(&mut b1, "orange".to_string());
}
