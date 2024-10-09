use diesel::{Queryable, Insertable};
use diesel::prelude::*;
use crate::database::schema::messages;
use crate::database::establish_connection;
use chrono::NaiveDateTime;

// This is the message struct used in your business logic
#[derive(Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

// This struct is for database interactions, including the id
#[derive(Debug, Queryable, Insertable, Clone)]
#[diesel(table_name = messages)]  // Updated attribute here
pub struct DbMessage {
    pub id: i32,                      // For primary key
    pub role: String,
    pub content: String,
    pub created_at: NaiveDateTime,
}

// This struct is used to create a new message without specifying the id
#[derive(Debug, Insertable)]
#[diesel(table_name = messages)] // Updated attribute here
pub struct NewDbMessage {
    pub role: String,
    pub content: String,
    pub created_at: NaiveDateTime,
}

// Save a message
pub fn save_message(role: &str, content: &str) {
    let new_message = NewDbMessage {
        role: role.to_string(),
        content: content.to_string(),
        created_at: chrono::Local::now().naive_utc(), // Adjust timezone as necessary
    };

    let mut connection = establish_connection();
    diesel::insert_into(messages::table)
        .values(&new_message)
        .execute(&mut connection)
        .expect("Error saving new message");
}

// Load history from the database
pub fn load_history() -> Vec<Message> {
    let mut connection = establish_connection();
    let db_messages: Vec<DbMessage> = messages::table.load(&mut connection).expect("Error loading messages");
    
    db_messages.into_iter().map(|msg| Message {
        role: msg.role,
        content: msg.content,
    }).collect() // Convert DbMessage to Message
}