use std::io::{self, Write};

pub fn get_input() -> String {
    print!("Please enter a string: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_owned()
}
