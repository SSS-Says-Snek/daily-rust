use crate::utils::get_input;

fn np3(n: u128) -> u128 {
    // nP3, where we are permutating n

    n * (n - 1) * (n - 2)
}

fn sign(n: u128) -> i128 {
    (-1_i128).pow(n as u32)
}

fn approx_pi(num_iter: u128) -> f64 {
    3.0 + (0..num_iter)
        .map(|n| sign(n) as f64 * 4.0 / np3(2 * n + 4) as f64)
        .sum::<f64>()
}

pub fn main() {
    println!();

    loop {
        let (input, break_out) = get_input("Enter num iterations to approximate");
        if break_out {
            break;
        };

        let num_iter: u128 = input.trim().parse().expect("That's not a number! >:(");
        let result = approx_pi(num_iter);
        println!("The result is {result}!");
    }
}
