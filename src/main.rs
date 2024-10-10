mod cli;
mod database;
mod handlers;
use dotenv::dotenv;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok(); // Load environment variables from .env file

    // Establish a database connection pool
    let pool = database::establish_pool();
    
    // Create the messages table if it does not exist
    let mut connection = database::get_connection(&pool);
    database::create_table_if_not_exists(&mut connection)?;

    println!("Enter 'exit' to end");
    loop {
        let input = cli::get_input();
        if input == "exit" {
            break;
        }

        // Pass the connection pool to get_response
        match handlers::get_response(&pool, &input).await {
            Some(response) => println!("Bot: {}", response),
            None => println!("Bot: Sorry, I couldn't understand that."),
        }
    }

    Ok(())
}