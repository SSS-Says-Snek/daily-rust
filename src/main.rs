mod first_prog;
mod fib;
mod utils;
mod taylor;
mod rects;
mod bsort;

use std::io::{self, Write};

fn main() {
    println!("Test rust stuff\n");
    println!("Progs to run: firstprog, fib, taylor, rects, bsort");

    loop {
        // Am lazy to see if it's possible without, so I'll just match everything
        let mut progname = String::new();

        print!("Enter prog to run: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut progname)
            .expect("Input Error");
        
        match progname.trim() {
            "firstprog" => first_prog::first_prog(),
            "fib" => fib::fib(),
            "taylor" => taylor::taylor(),
            "rects" => rects::rects(),
            "bsort" => bsort::bsort(),
            _ => println!("Does not exist!")
        }
    }
}
