use crate::utils::get_input;

pub fn main() {
    println!();

    loop {
        let (input, break_out) = get_input("Enter coefficient list, separated by spaces");
        if break_out {
            break;
        };

        let coeffs: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().expect("Not a number!"))
            .collect();

        let (input, break_out) =
            get_input("Enter a, where the linear binomial (x - a) is the divisor");
        if break_out {
            break;
        };

        let a: i32 = input.trim().parse().expect("Not a number!");

        let new_coeffs: Vec<i32> = coeffs
            .iter()
            .scan(0, |state, &x| {
                *state = *state * a + x;

                Some(*state)
            })
            .collect();

        println!("The coefficients are: {:?}", new_coeffs);
    }
}
