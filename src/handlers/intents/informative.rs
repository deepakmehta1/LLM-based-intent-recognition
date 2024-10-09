use crate::handlers::history::Message;
use crate::handlers::prompts::informative_prompt;

pub fn handle_informative_intent(query: &str) -> Message {
    println!("Imformative intent called");
    let prompt = informative_prompt();
    Message {
        role: prompt.role,
        content: format!("{}\n\nUser Query: {}", prompt.content, query),
    }
}
