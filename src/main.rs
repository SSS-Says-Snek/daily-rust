use std::io;
use std::io::Write;

fn main() {
    let mut name = String::new();
    print!("Enter your name: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut name)
        .expect("L bozo");
    
    println!("Your name is {name}")
}
