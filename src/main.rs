use std::io;
use std::io::Write;
use std::cmp::Ordering;

fn first_prog() {
    const MY_AGE: i32 = 15;

    println!();

    loop {
        let mut input = String::new();
        print!("Enter your age: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Input Error");
        
        if input.trim() == "quit" {
            println!("Bye!");
            break;
        }

        let age: i32 = input.trim().parse().expect("ENTER. A. NUMBER");
        println!("You're {age} years old!");

        match age.cmp(&MY_AGE) {
            Ordering::Less => println!("You're younger than me!"),
            Ordering::Greater => println!("You're older than me!"),
            Ordering::Equal => {
                println!("Hi");
                break;
            },
        }
    }
}

fn fib() {
    println!();

    loop {
        let mut input = String::new();
        print!("Enter nth term: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Input Error");
        
        if input.trim() == "quit" {
            println!("Bye!");
            break;
        }

        let term: u32 = input.trim().parse().expect("That's not a number! >:(");

        let mut result: u32 = 0;
        let mut a: u32 = 0;
        let mut b: u32 = 1;

        for _ in 0..term {
            result = a + b;
            a = b;
            b = result;
        }

        println!("fib({term}) is {result}!");
    }

    
}

fn main() {
    println!("Test rust stuff\n");
    println!("Progs to run: firstprog, fib");

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
            _ => println!("Does not exist!")
        }
    }
}
