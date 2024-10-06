mod cli;
mod handlers;

fn main() {
    let input = cli::get_input();
    handlers::print_input(&input);
}