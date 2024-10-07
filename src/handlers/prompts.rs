use crate::handlers::history::Message;

pub fn joke_prompt() -> Message {
    Message {
        role: "System".to_string(),
        content: [
            "You are a helpful assistant.",
            "Please greet the student a joke",
        ].join("\n").to_string()
    }
}