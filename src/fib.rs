use crate::utils::get_input;

pub fn main() {
    println!();

    loop {
        let (input, break_out) = get_input("Enter nth term");
        if break_out {break};

        let term: u32 = input.trim().parse().expect("That's not a number! >:(");

        let mut result: u64 = 0;
        let mut a: u64 = 0;
        let mut b: u64 = 1;

        for _ in 0..term {
            result = a + b;
            a = b;
            b = result;
        }

        println!("fib({term}) is {result}!");
    }
}