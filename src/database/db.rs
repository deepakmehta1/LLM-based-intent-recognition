// src/db.rs
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::QueryResult;
use diesel::sql_query;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    // Load environment variables from .env file
    dotenv().ok();

    // Get the DATABASE_URL from environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Establish a connection to the PostgreSQL database
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

// Function to create the messages table if it does not exist
pub fn create_table_if_not_exists(conn: &mut PgConnection) -> QueryResult<()> {
    sql_query(
        "CREATE TABLE IF NOT EXISTS messages (
            id SERIAL PRIMARY KEY,
            role VARCHAR NOT NULL,
            content TEXT NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT NOW()
        )",
    )
    .execute(conn)?;

    Ok(())
}
