use super::schema::messages;
use diesel::{Insertable, Queryable};

#[derive(Queryable, Insertable, Clone)]
#[diesel(table_name = messages)]
pub struct Message {
    pub id: i32,
    pub role: String,
    pub content: String,
    pub created_at: chrono::NaiveDateTime,
}
