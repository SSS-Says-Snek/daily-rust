mod first_prog;
mod fib;
mod utils;
mod taylor;
mod rects;

use std::io::{self, Write};

use first_prog::first_prog;
use fib::fib;
use taylor::taylor;
use rects::rects;

fn main() {
    println!("Test rust stuff\n");
    println!("Progs to run: firstprog, fib, taylor, rects");

    loop {
        // Am lazy to see if it's possible without, so I'll just match everything
        let mut progname = String::new();

        print!("Enter prog to run: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut progname)
            .expect("Input Error");
        
        match progname.trim() {
            "firstprog" => first_prog(),
            "fib" => fib(),
            "taylor" => taylor(),
            "rects" => rects(),
            _ => println!("Does not exist!")
        }
    }
}
