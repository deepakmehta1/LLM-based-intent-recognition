use diesel::{Queryable, Insertable};
use super::schema::messages;

#[derive(Queryable, Insertable, Clone)]
#[diesel(table_name = messages)]
pub struct Message {
    pub id: i32,                      
    pub role: String,
    pub content: String,
    pub created_at: chrono::NaiveDateTime,
}