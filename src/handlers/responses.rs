use crate::database::db::DbPool;
use crate::handlers::history::{load_history, save_message, Message}; 
use crate::handlers::intents::{handle_empathetic_intent, handle_informative_intent, handle_others_intent, handle_task_oriented_intent};
use crate::handlers::prompts::basic_prompt;
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{
    self, ChatCompletionMessage, ChatCompletionRequest, Content, Function, FunctionParameters,
    JSONSchemaDefine, JSONSchemaType, MessageRole, Tool, ToolChoiceType, ToolType,
};
use openai_api_rs::v1::common::GPT4_O;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

// Add a pool parameter to get_response
pub async fn get_response(pool: &DbPool, input: &str) -> Option<String> {
    get_response_impl(pool, input, OpenAIClient::new).await
}

async fn get_response_impl<F>(pool: &DbPool, input: &str, client_factory: F) -> Option<String>
where
    F: Fn(String) -> OpenAIClient,
{
    let openai_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let client = client_factory(openai_key);

    // Collect the message history from the database using the connection pool
    let history = load_history(pool);

    // Start with the basic prompt in message format
    let mut messages: Vec<Message> = history;
    messages.insert(0, basic_prompt()); // Add the basic prompt at the start

    // Add the user's input to history
    let user_message = Message {
        role: "user".to_string(),
        content: input.to_string(),
    };
    messages.push(user_message.clone());
    save_message(pool, "user", &user_message.content); // Pass pool to save_message

    // Convert `Message` to `ChatCompletionMessage`
    let chat_messages: Vec<ChatCompletionMessage> = messages.iter().map(|msg| {
        ChatCompletionMessage {
            role: match msg.role.as_str() {
                "user" => MessageRole::user,
                "assistant" => MessageRole::assistant,
                "system" => MessageRole::system,
                _ => MessageRole::user, // Default to user if role is unrecognized
            },
            content: Content::Text(msg.content.clone()),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }).collect();

    let mut properties = HashMap::new();
    properties.insert(
        "intent".to_string(),
        Box::new(JSONSchemaDefine {
            schema_type: Some(JSONSchemaType::String),
            description: Some("Specify the type of intent to identify. Options: informative, empathetic, task_oriented, others.".to_string()),
            ..Default::default()
        }),
    );

    let req = ChatCompletionRequest::new(GPT4_O.to_string(), chat_messages.clone())
        .tools(vec![Tool {
            r#type: ToolType::Function,
            function: Function {
                name: String::from("get_intent"),
                description: Some(String::from(
                    "Identify the type of intent based on the user's intent. It includes the following categories:
                    1. informative: Triggered when the user asks for specific information or facts. Examples include: 'What is the capital of France?', 'Can you tell me about the solar system?', 'How does photosynthesis work?'.
                    2. empathetic: Triggered when the user shares personal feelings, concerns, or experiences. Examples include: 'I've had a tough day today.', 'I'm feeling really stressed out about work.', 'I need someone to talk to about my problems.'.
                    3. task_oriented: Triggered when the user wants to accomplish a specific task. Examples include: 'Can you set a reminder for my meeting at 2 PM?', 'Help me book a table at an Italian restaurant.', 'Find me a good laptop under $1000.'.
                    4. others: Triggered when the intent is not among informative, empathetic, or task-oriented. Examples include: 'Hello, how are you?', 'Goodbye', 'Good Morning.'"
                )),
                parameters: FunctionParameters {
                    schema_type: JSONSchemaType::Object,
                    properties: Some(properties),
                    required: Some(vec![String::from("intent")]),
                },
            },
        }])
        .tool_choice(ToolChoiceType::Auto);

    let result = client.chat_completion(req).await.ok()?;

    if let Some(finish_reason) = &result.choices[0].finish_reason {
        match finish_reason {
            chat_completion::FinishReason::tool_calls => {
                #[derive(Deserialize, Serialize)]
                struct Intent { intent: String }

                let tool_calls = result.choices[0].message.tool_calls.as_ref().unwrap();
                for tool_call in tool_calls {
                    let arguments = tool_call.function.arguments.clone().unwrap();
                    let intent: Intent = serde_json::from_str(&arguments).ok()?;

                    // Generate response based on the intent
                    let response_message = match intent.intent.as_str() {
                        "informative" => handle_informative_intent(input),
                        "empathetic" => handle_empathetic_intent(input),
                        "task_oriented" => handle_task_oriented_intent(input),
                        _ => handle_others_intent(input),
                    };

                    // Update the first message with the specific intent prompt
                    messages[0] = response_message;

                    // Convert updated Messages to ChatCompletionMessages
                    let updated_chat_messages: Vec<ChatCompletionMessage> = messages.iter().map(|msg| {
                        ChatCompletionMessage {
                            role: match msg.role.as_str() {
                                "user" => MessageRole::user,
                                "assistant" => MessageRole::assistant,
                                "system" => MessageRole::system,
                                _ => MessageRole::user,
                            },
                            content: Content::Text(msg.content.clone()),
                            name: None,
                            tool_calls: None,
                            tool_call_id: None,
                        }
                    }).collect();

                    // Create a new request with updated messages
                    let req = ChatCompletionRequest::new(GPT4_O.to_string(), updated_chat_messages);

                    let result = client.chat_completion(req).await.ok()?;
                    if let Some(response_content) = result.choices[0].message.content.clone() {
                        save_message(pool, "assistant", &response_content); // Pass pool
                        return Some(response_content);
                    }
                }
            }
            _ => {}
        }
    }

    if let Some(response_content) = result.choices[0].message.content.clone() {
        save_message(pool, "assistant", &response_content); // Pass pool
        return Some(response_content);
    }
    None
}