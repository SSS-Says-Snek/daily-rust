use std::ops::{Mul, Sub};

use crate::utils::get_input;

fn determinant<T>(a: T, b: T, c: T, d: T) -> T
where
    T: Mul<Output = T> + Sub<Output = T>,
{
    a * d - b * c
}

fn get_equation(input: &str) -> (i32, i32, i32) {
    let eq: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Not a number!"))
        .collect();

    if eq.len() != 3 {
        panic!("Input is not three numbers separated by spaces");
    }

    (eq[0], eq[1], eq[2])
}

pub fn main() {
    println!();

    loop {
        let (input, break_out) =
            get_input("Enter coefficients and constant of the first equation, separated by spaces");
        if break_out {
            break;
        };

        let (a, b, c) = get_equation(&input);

        let (input, break_out) = get_input(
            "Enter coefficients and constant of the second equation, separated by spaces",
        );
        if break_out {
            break;
        };

        let (d, e, f) = get_equation(&input);
        let denominator = determinant(a, b, d, e);

        if denominator == 0 {
            println!("No solution!");
            continue;
        }

        let dx = determinant(c, b, f, e);
        let dy = determinant(a, c, d, f);

        let x = dx as f64 / denominator as f64;
        let y = dy as f64 / denominator as f64;

        println!();
        println!("Input: {a}x + {b}y = {c}");
        println!("       {d}x + {e}y = {f}");
        println!("Solutions: x = {x}, y = {y}");
    }
}
