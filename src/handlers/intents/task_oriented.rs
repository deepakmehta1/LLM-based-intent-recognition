use crate::handlers::history::Message;
use crate::handlers::prompts::basic_prompt;

pub fn handle_task_oriented_intent(query: &str) -> Message {
    println!("Task Oriented intent called");
    let prompt = basic_prompt();
    Message {
        role: prompt.role,
        content: format!("{}\n\nUser Query: {}", prompt.content, query),
    }
}
