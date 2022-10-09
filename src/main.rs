use std::io;
use std::io::Write;
use std::cmp::Ordering;

fn main() {
    const MY_AGE: i32 = 15;

    loop {
        let mut input = String::new();
        print!("Enter your age: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("L bozo");
        
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
