pub mod prompts;
pub mod responses;
pub mod history;
pub mod intents;

pub use prompts::joke_prompt;
pub use responses::get_response;
pub use history::{save_message, load_history, Message};
pub use intents::{
    handle_informative_intent,
    handle_empathetic_intent,
    handle_task_oriented_intent,
    handle_others_intent,
    handle_intent,
};