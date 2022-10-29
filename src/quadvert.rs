use crate::utils::get_input;

fn quad(x: f64, a: f64, b: f64, c: f64) -> f64 {
    a * x.powi(2) + b * x + c
}

pub fn main() {
    println!();

    loop {
        let (input, break_out) = get_input("Enter a, b, c, separated by spaces");
        if break_out {
            break;
        };

        let coeffs: Vec<f64> = input
            .split_whitespace()
            .map(|x| x.parse().expect("Not a number!"))
            .collect();

        if coeffs.len() != 3 {
            panic!("Input is not three numbers separated by spaces");
        }

        let (a, b, c) = (coeffs[0], coeffs[1], coeffs[2]);
        let axis_of_sym = -b as f64 / (2.0 * a as f64);
        let vertex = (axis_of_sym, quad(axis_of_sym, a, b, c));

        println!("The vertex is: ({}, {})", vertex.0, vertex.1);
    }
}
