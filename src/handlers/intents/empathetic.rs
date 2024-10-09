use crate::handlers::prompts::empathetic_prompt;
use crate::handlers::history::Message;

pub fn handle_empathetic_intent(query: &str) -> Message {
    let prompt = empathetic_prompt();
    // Logic to include the user's query if needed.
    Message {
        role: prompt.role,
        content: format!("{}\n\nUser Query: {}", prompt.content, query)
    }
}