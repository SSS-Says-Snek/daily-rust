mod bsort;
mod fib;
mod first_prog;
mod rects;
mod secret_message;
mod taylor;
mod utils;
mod deriv;

use std::io::{self, Write};

fn main() {
    println!("Test rust stuff\n");
    println!("Progs to run: firstprog, fib, taylor, rects, bsort, secretmessage, deriv");

    loop {
        // Am lazy to see if it's possible without, so I'll just match everything
        let mut progname = String::new();

        print!("Enter prog to run: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut progname).expect("Input Error");

        match progname.trim() {
            "firstprog" => first_prog::main(),
            "fib" => fib::main(),
            "taylor" => taylor::main(),
            "rects" => rects::main(),
            "bsort" => bsort::main(),
            "secretmessage" => secret_message::main(),
            "deriv" => deriv::main(),
            _ => println!("Does not exist!"),
        }
    }
}
