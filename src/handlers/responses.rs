use std::env;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest, ToolType, Tool, Function, FunctionParameters, JSONSchemaDefine, JSONSchemaType, ToolChoiceType, ChatCompletionMessage, MessageRole, Content};
use openai_api_rs::v1::common::GPT4_O;
use crate::handlers::{save_message, load_history, Message};
use crate::handlers::prompts::basic_prompt;
use crate::handlers::intents::{handle_informative_intent, handle_empathetic_intent};

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

    // Start with the basic prompt in message format
    let mut messages: Vec<Message> = vec![basic_prompt()];
    messages.extend(history);

    // Add the user's input to history
    messages.push(Message {
        role: "user".to_string(),
        content: input.to_string(),
    });
    save_message("user", input);

    // Convert Message to ChatCompletionMessage
    let chat_messages: Vec<ChatCompletionMessage> = messages.iter().map(|msg| {
        ChatCompletionMessage {
            role: match msg.role.as_str() {
                "user" => MessageRole::user,
                "assistant" => MessageRole::assistant,
                "system" => MessageRole::system,
                _ => MessageRole::user,  // Default to user if role is unrecognized
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

    match result.choices[0].finish_reason {
        Some(chat_completion::FinishReason::tool_calls) => {
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
                    _ => basic_prompt(),
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
                    save_message("assistant", &response_content);
                    return Some(response_content);
                }
            }
        }
        _ => {}
    }

    None
}