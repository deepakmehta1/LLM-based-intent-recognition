use crate::handlers::history::Message;

pub fn basic_prompt() -> Message {
    Message {
        role: "System".to_string(),
        content: [
            "You are a helpful assistant.",
            "Your job is help user with whatever user needs.",
        ].join("\n").to_string()
    }
}