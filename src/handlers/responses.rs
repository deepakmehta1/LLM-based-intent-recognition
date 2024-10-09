use std::env;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4_O;
use crate::handlers::{basic_prompt, save_message, load_history, Message};
use crate::handlers::intents::{handle_informative_intent, handle_empathetic_intent, handle_task_oriented_intent, handle_others_intent};

pub async fn get_response(input: &str) -> Option<String> {
    get_response_impl(input, OpenAIClient::new).await
}

async fn get_response_impl<F>(input: &str, client_factory: F) -> Option<String>
where
    F: Fn(String) -> OpenAIClient,
{
    let openai_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let client = client_factory(openai_key);

    // Collect history and convert it to a suitable format for OpenAI request
    let history = load_history();
    
    // Prepend the joke_prompt to the history
    let mut messages = vec![basic_prompt()];
    messages.extend(history);

    // Append the user message to the history for the current request
    messages.push(Message {
        role: "user".to_string(),
        content: input.to_string(),
    });
    save_message("user", input);
    //println!("history: {:?}", messages);

    let messages: Vec<chat_completion::ChatCompletionMessage> = messages.iter().map(|msg| {
        chat_completion::ChatCompletionMessage {
            role: match msg.role.as_str() {
                "user" => chat_completion::MessageRole::user,
                "assistant" => chat_completion::MessageRole::assistant,
                "system" => chat_completion::MessageRole::system,
                _ => chat_completion::MessageRole::user,  // Default to user if role is unrecognized
            },
            content: chat_completion::Content::Text(msg.content.clone()),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }).collect();

    let mut properties = HashMap::new();
    properties.insert(
        "intent".to_string(),
        Box::new(chat_completion::JSONSchemaDefine {
            schema_type: Some(chat_completion::JSONSchemaType::String),
            description: Some(
                "Specify the type of intent to identify. Options: informative, empathetic, task_oriented, others."
                .to_string(),
            ),
            ..Default::default()
        }),
    );

    // Create the request with the history and new input included
    let req = ChatCompletionRequest::new(
        GPT4_O.to_string(),
        messages,
    )
    .tools(vec![chat_completion::Tool {
        r#type: chat_completion::ToolType::Function,
        function: chat_completion::Function {
            name: String::from("get_intent"),
            description: Some(String::from(
                "Identify the type of intent based on the user's intent. It includes the following categories: 
                1. informative: Triggered when the user asks for specific information or facts. Examples include: 'What is the capital of France?', 'Can you tell me about the solar system?', 'How does photosynthesis work?'. 
                2. empathetic: Triggered when the user shares personal feelings, concerns, or experiences. Examples include: 'I've had a tough day today.', 'I'm feeling really stressed out about work.', 'I need someone to talk to about my problems.'. 
                3. task_oriented: Triggered when the user wants to accomplish a specific task. Examples include: 'Can you set a reminder for my meeting at 2 PM?', 'Help me book a table at an Italian restaurant.', 'Find me a good laptop under $1000.'. 
                4. others: Triggered when the intent is not among informative, empathetic, or task-oriented. Examples include: 'Hello, how are you?', 'Goodbye', 'Good Morning.'"
            )),
            parameters: chat_completion::FunctionParameters {
                schema_type: chat_completion::JSONSchemaType::Object,
                properties: Some(properties),
                required: Some(vec![String::from("intent")]),
            },
        },
    }])
    .tool_choice(chat_completion::ToolChoiceType::Auto);

    let result = client.chat_completion(req).await.ok()?;

    match result.choices[0].finish_reason {
        None => println!("No finish_reason"),
        Some(chat_completion::FinishReason::stop) => println!("Stop"),
        Some(chat_completion::FinishReason::length) => println!("Length"),
        Some(chat_completion::FinishReason::tool_calls) => {
            println!("ToolCalls");
            #[derive(Deserialize, Serialize)]
            struct Intent {
                intent: String,
            }
            let tool_calls = result.choices[0].message.tool_calls.as_ref().unwrap();
            for tool_call in tool_calls {
                let _ = tool_call.function.name.clone().unwrap();
                let arguments = tool_call.function.arguments.clone().unwrap();
                let intent: Intent = serde_json::from_str(&arguments).ok()?;
                match intent.intent.as_str() {
                    "informative" => {
                        let response = handle_informative_intent(input);
                        println!("Response: {}", response);
                    }
                    "empathetic" => {
                        let response = handle_empathetic_intent(input);
                        println!("Response: {}", response);
                    }
                    "task_oriented" => {
                        let response = handle_task_oriented_intent(input);
                        println!("Response: {}", response);
                    }
                    "others" => {
                        let response = handle_others_intent(input);
                        println!("Response: {}", response);
                    }
                    _ => println!("Unknown intent: {}", intent.intent),
                }
            }
        }
        Some(chat_completion::FinishReason::content_filter) => println!("ContentFilter"),
        Some(chat_completion::FinishReason::null) => println!("Null"),
    }

    // Extract and save the response
    if let Some(response_content) = result.choices[0].message.content.clone() {
        save_message("assistant", &response_content);
        return Some(response_content);
    }

    None
}