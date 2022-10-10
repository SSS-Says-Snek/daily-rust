mod first_prog;
mod fib;
mod utils;
mod taylor;
mod rects;

use std::io;
use std::io::Write;

use crate::first_prog::first_prog;
use crate::fib::fib;
use crate::taylor::taylor;
use crate::rects::rects;

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
