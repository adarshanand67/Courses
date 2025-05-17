mod mymod;
use mymod::module1;

fn main() {
    let _s = module1::ModStruct {};
    module1::module_fn1();
    println!("Main function executed.");
}

// Read about rust libraries