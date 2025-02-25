// Learning about generics, traits

use num_traits::ToPrimitive;

fn solve<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {
    let a_64 = a.to_f64().unwrap();
    let b_64 = b.to_f64().unwrap();

    (a_64.powi(2) + b_64.powi(2)).sqrt()
}

fn main() {
    let a: i32 = 3;
    let b: f64 = 4.0;

    let c = solve(a, b);
    println!(
        "The hypotenuse of a right triangle with sides {} and {} is {}",
        a, b, c
    );
}
