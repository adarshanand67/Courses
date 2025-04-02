pub trait Describable {
    fn describe(&self) -> String;

    fn print_description(&self) {
        println!("{}", self.describe());
    }
}

impl Describable for super::structs::Rectangle {
    fn describe(&self) -> String {
        format!(
            "Rectangle with width {} and height {}",
            self.width, self.height
        )
    }
}

pub fn traits_example() {
    let rect: super::structs::Rectangle = super::structs::Rectangle {
        width: 15,
        height: 25,
    };
    rect.print_description();
}
