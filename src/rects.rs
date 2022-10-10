use std::cmp::{min, max};

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

    fn colliderect(&self, other: &Rect) -> bool {
        min(self.x, self.x + self.width) < max(other.x, other.x + other.width) && 
        min(self.y, self.y + self.height) < max(other.y, other.y + other.height) && 
        max(self.x, self.x + self.width) > min(other.x, other.x + other.width) &&
        max(self.y, self.y + self.height) > min(other.y, other.y + other.height)
    }

    fn move_ip(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

pub fn rects() {
    println!();

    let mut example1 = Rect {
        x: 4, y: 5, width: 10, height: 20
    };

    let example2 = Rect {
        x: 5, y: 6, width: 6, height: 15
    };

    let example3 = Rect {
        x: 1000, y: 1000, width: 10, height: 20
    };

    let contains = example1.contains(&example2);
    let colliderect = example1.colliderect(&example3);
    println!("Example1 contains example2: {contains}");
    println!("Example1 intersects with example3: {colliderect}");

    example1.move_ip(1, 4);
    println!("After moving x 1 and y 4, example1's x and y pos is now: {} {}", example1.x, example1.y);
}