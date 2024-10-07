mod cli;
mod handlers;
use dotenv::dotenv;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();  // Load environment variables from .env file

    println!("Enter 'exit' to end");
    loop {
        let input = cli::get_input();
        if input == "exit" {
            break;
        }

        // Handle the Option<String> returned by get_response
        match handlers::get_response(&input).await {
            Some(response) => println!("Bot: {}", response),
            None => println!("Bot: Sorry, I couldn't understand that."),
        }
    }

    Ok(())
}