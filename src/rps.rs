use rand::seq::SliceRandom;

use crate::utils::get_input;

pub fn main() {
    println!();

    let options = ["r", "p", "s"];

    let mut wins = 0;
    let mut losses = 0;

    loop {
        let (input, break_out) = get_input("Enter move (r, p, s)");
        if break_out {
            return;
        };

        let input = input.trim();
        if !["r", "p", "s"].contains(&input) {
            println!("That's not a valid option!");
            continue;
        }

        let comp_move: &str = options.choose(&mut rand::thread_rng()).unwrap();
        println!("The computer chose {}", match comp_move {
            "r" => "rock",
            "s" => "scissors",
            "p" => "paper",
            _ => "invalid"
        });

        match format!("{}{}", input, comp_move).as_str() {
            "rs" | "pr" | "sp" => {
                println!("You won!");
                wins += 1;
            },
            "rr" | "pp" | "ss" => {
                println!("It's a draw!");
                wins += 1;
                losses += 1;
            }
            _ => {
                println!("You lost!");
                losses += 1;
            }
        }

        println!("Wins: {wins}, losses: {losses}");
    }
}