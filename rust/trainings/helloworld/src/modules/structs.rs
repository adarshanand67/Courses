pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub struct Point3D(pub i32, pub i32, pub i32);

pub fn structs_example() {
    let rect: Rectangle = Rectangle {
        width: 10,
        height: 20,
    };
    println!("Rectangle area: {}", rect.area());

    let point = Point3D(1, 2, 3);
    println!("Point3D({}, {}, {})", point.0, point.1, point.2);
}
