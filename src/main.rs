mod bsort;
mod deriv;
mod ecrates;
mod fib;
mod first_prog;
mod pi;
mod product;
mod quadvert;
mod rects;
mod rps;
mod secret_message;
mod synthdiv;
mod syslin;
mod taylor;
mod utils;

use colored::Colorize;
use std::io::{self, Write};

fn main() {
    println!("\n{}", "Test rust stuff\n".bright_yellow());
    println!(
        "{} {}",
        "Progs to run:".bright_green(),
        "firstprog, fib, taylor, rects, bsort, secretmessage, deriv, product, rps, ecrates, pi, quadvert, synthdiv, syslin"
            .bright_cyan()
    );

    loop {
        // Am lazy to see if it's possible without, so I'll just match everything
        let mut progname = String::new();

        print!("{}", "Enter prog to run: ".bright_green());
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
            "product" => product::main(),
            "rps" => rps::main(),
            "ecrates" => ecrates::main(),
            "pi" => pi::main(),
            "quadvert" => quadvert::main(),
            "synthdiv" => synthdiv::main(),
            "syslin" => syslin::main(),
            _ => println!("{}", "Does not exist!".bright_red()),
        }
    }
}
