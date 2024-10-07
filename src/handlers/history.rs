use std::sync::Mutex;
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

lazy_static! {
    static ref HISTORY: Mutex<Vec<Message>> = Mutex::new(Vec::new());
}

pub fn save_message(role: &str, content: &str) {
    let mut history = HISTORY.lock().unwrap();
    history.push(Message {
        role: role.to_string(),
        content: content.to_string(),
    });
}

pub fn load_history() -> Vec<Message> {
    let history = HISTORY.lock().unwrap();
    history.clone()
}