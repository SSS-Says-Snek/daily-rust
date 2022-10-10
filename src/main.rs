use std::io;
use std::io::Write;
use std::cmp::Ordering;

fn get_input(input_txt: &str) -> (String, bool) {
    let mut input = String::new();
    print!("{input_txt}: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Input Error");

    let mut break_out = false;
    if ["quit", "q"].contains(&input.trim()) {
        println!("Bye!\n");
        break_out = true;
    }
    
    (input, break_out)
}

fn factorial(num: u128) -> u128 {
    match num {
        0  => 1,
        1.. => (1..num+1).product(),
    }
}

fn first_prog() {
    const MY_AGE: i32 = 15;

    println!();

    loop {
        let (input, break_out) = get_input("Enter age");
        if break_out {break};

        let age: i32 = input.trim().parse().expect("That's not a number! >:(");
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

fn taylor() {
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

fn main() {
    println!("Test rust stuff\n");
    println!("Progs to run: firstprog, fib, taylor");

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
            _ => println!("Does not exist!")
        }
    }
}
