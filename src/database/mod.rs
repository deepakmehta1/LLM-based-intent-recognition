pub mod db;
pub mod models;
pub mod schema;

pub use db::{create_table_if_not_exists, establish_connection};
