// src/db.rs
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::result::QueryResult;
use diesel::sql_query;
use dotenv::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

// Establish a connection pool
pub fn establish_pool() -> DbPool {
    dotenv().ok(); // Load environment variables from `.env` file

    // Get the DATABASE_URL from environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a connection manager for PostgreSQL
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    // Build the connection pool
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

// Get a connection from the pool
pub fn get_connection(pool: &DbPool) -> PooledConnection<ConnectionManager<PgConnection>> {
    pool.get()
        .expect("Failed to get a connection from the pool")
}

// Function to create the messages table if it does not exist
pub fn create_table_if_not_exists(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> QueryResult<()> {
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
