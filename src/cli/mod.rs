use std::io;

pub fn get_input() -> String {
    println!("Please enter a string:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_owned()
}