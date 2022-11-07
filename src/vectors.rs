use std::ops::{Add, Div, Mul, Sub};
use std::fmt::{Display, Formatter, Error};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
struct Vector2(f64, f64);

impl Vector2 {
    fn magnitude(&self) -> f64 {
        self.0.hypot(self.1)
    }
    
    fn normalize(&mut self) {
        let magnitude = self.magnitude();
        self.0 /= magnitude;
        self.1 /= magnitude;
    }
    
    fn cross(&self, other: &Self) -> f64 {
        determinant(self.0, self.1, other.0, other.1)
    }
    
    fn angle_to(&self, other: &Self) -> f64 {
        (self.cross(other).abs() / (self.magnitude() * other.magnitude())).asin()
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl Mul for Vector2 {
    type Output = f64;
    
    fn mul(self, other: Self) -> Self::Output {
        self.0 * other.0 + self.1 * other.1
    }
}

impl Div<f64> for Vector2 {
    type Output = Self;
    
    fn div(self, other: f64) -> Self::Output {
        Self(self.0 / other, self.1 / other)
    }
}

impl Display for Vector2 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({}, {})", self.0, self.1)
    }
}

fn determinant<T>(a: T, b: T, c: T, d: T) -> T
where
    T: Mul<Output = T> + Sub<Output = T>,
{
    a * d - b * c
}

fn larger<'a>(a: &'a Vector2, b: &'a Vector2) -> &'a Vector2 {
    if a.magnitude() > b.magnitude() {
        return a
    }
    
    b
}

pub fn main() {
    let smth = Vector2(4.0, 0.0);
    let mut a = Vector2(4.0, 4.0);
    let b = Vector2(3.0, 4.0);
    let res = a + b;
    let largest = larger(&a, &b);
    let a_angle_to_smth = a.angle_to(&smth).to_degrees();

    println!("a: {a}, b: {b}, res: {res}");
    println!("Cross product of a and b: {}", a.cross(&b));
    println!("Angle of a to smth: {a_angle_to_smth} deg");
    println!("Largest of a and b: {largest}");
    
    a.normalize();
    println!("a normalized: {a}\na magnitude: {}", a.magnitude())
}
