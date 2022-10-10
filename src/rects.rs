struct Rect {
    x: i32, y: i32,
    width: i32, height: i32
}

impl Rect {
    fn contains(&self, other: &Rect) -> bool {
        (self.x <= other.x) && (self.y <= other.y) &&
        (self.x + self.width >= other.x + other.width) &&
        (self.y + self.height >= other.y + other.height) &&
        (self.x + self.width > other.x) &&
        (self.y + self.height > other.y)
    }
}

pub fn rects() {
    println!();

    let example1 = Rect {
        x: 4, y: 5, width: 10, height: 20
    };

    let example2 = Rect {
        x: 5, y: 6, width: 6, height: 15
    };

    let contains = example1.contains(&example2);
    println!("Example1 contains example2: {contains}");
}