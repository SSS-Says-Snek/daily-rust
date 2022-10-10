use crate::utils::get_input;

fn factorial(num: u128) -> u128 {
    match num {
        0  => 1,
        1.. => (1..num+1).product(),
    }
}

pub fn taylor() {
    println!();

    loop {
        let (input, break_out) = get_input("Enter nth iteration for sin(x)");
        if break_out {break};
        let num_iter: i32 = input.trim().parse().expect("That's not a number! >:(");

        let (input, break_out) = get_input("Enter radians");
        if break_out {break};
        let angle: f64 = input.trim().parse().expect("That's not a number! >:(");

        let mut result = 0.0;

        for i in 0..num_iter {
            let sign = (2 * ((i + 1) % 2) - 1) as f64;
            let exp = (2 * i + 1) as i32;

            result += sign * angle.powi(exp) / factorial(exp as u128) as f64;
        }

        println!("The result is: {result}");
    }
}