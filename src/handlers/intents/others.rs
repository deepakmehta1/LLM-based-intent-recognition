use crate::handlers::prompts::basic_prompt;
use crate::handlers::history::Message;

pub fn handle_others_intent(query: &str) -> Message {
    println!("Others intent called");
    let prompt = basic_prompt();
    // Logic to include the user's query if needed.
    Message {
        role: prompt.role,
        content: format!("{}\n\nUser Query: {}", prompt.content, query)
    }
}