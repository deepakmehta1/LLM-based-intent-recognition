use crate::cli::get_input;
use crate::handlers::print_input;

fn main() {
    let input = get_input();
    print_input(&input);
}

mod cli {
    use std::io;
    pub fn get_input() -> String {
        println!("Please enter a string:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_owned()
    }
}

mod handlers {
    pub fn print_input(input: &str) {
        println!("{}", input);
    }
}