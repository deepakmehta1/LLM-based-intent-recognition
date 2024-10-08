use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Intent {
    pub name: String,
    pub description: String,
    pub examples: Vec<String>,
}

pub fn get_intents() -> Vec<Intent> {
    vec![
        Intent {
            name: "informative".to_string(),
            description: "Triggered when the user asks for specific information or facts.".to_string(),
            examples: vec![
                "What is the capital of France?".to_string(),
                "Can you tell me about the solar system?".to_string(),
                "How does photosynthesis work?".to_string(),
            ],
        },
        Intent {
            name: "empathetic".to_string(),
            description: "Triggered when the user shares personal feelings, concerns, or experiences.".to_string(),
            examples: vec![
                "I've had a tough day today.".to_string(),
                "I'm feeling really stressed out about work.".to_string(),
                "I need someone to talk to about my problems.".to_string(),
            ],
        },
        Intent {
            name: "task_oriented".to_string(),
            description: "Triggered when the user wants to accomplish a specific task.".to_string(),
            examples: vec![
                "Can you set a reminder for my meeting at 2 PM?".to_string(),
                "Help me book a table at an Italian restaurant.".to_string(),
                "Find me a good laptop under $1000.".to_string(),
            ],
        },
        Intent {
            name: "others".to_string(),
            description: "Triggered when the intent is not among informative, empathetic or task_oriented.".to_string(),
            examples: vec![
                "Hello, how are you?".to_string(),
                "Goodbye".to_string(),
                "Good Morning".to_string(),
            ],
        },
    ]
}
