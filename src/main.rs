mod cli;
mod handlers;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();  // Load environment variables from .env file

    println!("Enter 'exit' to end");
    loop {
        let input = cli::get_input();
        if input == "exit" {
            break;
        }
        if let Err(e) = handlers::print_response(&input).await {
            eprintln!("Error: {:?}", e);
        }
    }
}