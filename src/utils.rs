use std::io::{self, Write};

pub fn get_input(input_txt: &str) -> (String, bool) {
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