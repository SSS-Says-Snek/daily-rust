pub trait Shape {
    fn area(&self) -> i32;
}

struct Square {
    width: i32
}

impl Shape for Square {
    fn area(&self) {
        width ** 2;
    }
}

struct Triangle {
    base: i32,
    height: i32
}

impl Shape for Triangle {
    fn area(&self) {
        base * height / 2
    }
}

fn print_area(shape: &impl Shape) {
    println!("This shape's area is {shape.area()}!");
}