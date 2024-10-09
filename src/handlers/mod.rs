pub mod prompts;
pub mod responses;
pub mod history;
pub mod intents;

pub use prompts::basic_prompt;
pub use responses::get_response;
pub use history::{save_message, load_history, Message};