use crate::handlers::prompts::basic_prompt;
use crate::handlers::history::Message;

pub fn handle_task_oriented_intent(query: &str) -> Message {
    println!("Task Oriented intent called");
    let prompt = basic_prompt();
    // Logic to include the user's query if needed.
    Message {
        role: prompt.role,
        content: format!("{}\n\nUser Query: {}", prompt.content, query)
    }
}