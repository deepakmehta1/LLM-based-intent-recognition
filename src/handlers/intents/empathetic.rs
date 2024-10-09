use crate::handlers::history::Message;
use crate::handlers::prompts::empathetic_prompt;

pub fn handle_empathetic_intent(query: &str) -> Message {
    println!("Empathetic intent called");
    let prompt = empathetic_prompt();
    Message {
        role: prompt.role,
        content: format!("{}\n\nUser Query: {}", prompt.content, query),
    }
}
