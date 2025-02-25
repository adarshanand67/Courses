use std::io;

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

fn main() {
    let mut weight: String = String::new();
    println!("Enter your weight (kg): ");
    io::stdin()
        .read_line(&mut weight)
        .expect("Failed to read line");

    let mars_weight: f32 = match weight.trim().parse() {
        Ok(parsed_weight) => calculate_weight_on_mars(parsed_weight),
        Err(_) => {
            println!("Invalid weight entered. Please enter a valid number.");
            return;
        }
    };
    println!("Weight on Mars: {}", mars_weight);
}
