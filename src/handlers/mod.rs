pub mod prompts;
pub mod responses;
pub mod history;

// Re-exporting specific functions for easier access
pub use prompts::create_prompt;
pub use responses::get_response;
pub use history::{save_message, load_history, Message};