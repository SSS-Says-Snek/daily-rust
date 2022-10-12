// I always try to implement my first useful Python program in any language I'm learning!
// Time to implement it in Rust

use crate::utils::get_input;

fn get_half_letters(msg: &str, mode: u8) -> Vec<char> {
    let mut result = Vec::new();

    // Goofy ahh graphemes
    for (i, c) in msg.chars().enumerate() {
        if (i % 2) as u8 == mode {result.push(c)}
    }

    result
}

fn swap_letters(msg: &str) -> String {
    let mut result = String::new();
    let evens = get_half_letters(msg, 0);
    let mut odds = get_half_letters(msg, 1);

    // If the msg len is odd, assert_eq!(odds.len(), evens.len() - 1). So, we add an x to odds
    if msg.len() % 2 == 1 {odds.push('x')}

    // Effectively swaps the places of odd and even letter
    for i in 0..odds.len() {
        result.push(odds[i]);
        result.push(evens[i]);
    }

    result
}

fn encdec(mode: &str) {
    // Deref coerces into a &str
    let (input, break_out) = get_input(&format!("Enter message to {mode}"));
    if break_out {return};

    let input = input.trim();

    swap_letters(&input);
}

pub fn main() {
    println!("\nWelcome to \"The Encryptor Decryptor\"!");

    loop {
        let (input, break_out) = get_input("What do you want to do? (enter encrypt/decrypt)"); 
        if break_out {break};

        let input = input.trim();
        if !["encrypt", "decrypt"].contains(&input) {
            println!("That's not a valid option!");
            continue;
        }

        encdec(&input);
    }
}

