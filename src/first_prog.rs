use std::cmp::Ordering;

use crate::utils::get_input;

pub fn first_prog() {
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