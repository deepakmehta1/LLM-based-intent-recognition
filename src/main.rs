mod cli;
mod handlers;

fn main() {
    println!("Enter exit to end");
    loop {
        let input = cli::get_input();
        if input == "exit" {
            break
        }
        handlers::print_input(&input);
    }
}