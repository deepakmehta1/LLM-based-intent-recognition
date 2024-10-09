use crate::handlers::prompts::informative_prompt;
use crate::handlers::history::Message;

pub fn handle_informative_intent(query: &str) -> Message {
    println!("Imformative intent called");
    let prompt = informative_prompt();
    // Logic to include the user's query if needed.
    Message {
        role: prompt.role,
        content: format!("{}\n\nUser Query: {}", prompt.content, query)
    }
}