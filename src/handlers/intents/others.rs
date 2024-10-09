use crate::handlers::history::Message;
use crate::handlers::prompts::basic_prompt;

pub fn handle_others_intent(query: &str) -> Message {
    println!("Others intent called");
    let prompt = basic_prompt();
    Message {
        role: prompt.role,
        content: format!("{}\n\nUser Query: {}", prompt.content, query),
    }
}
