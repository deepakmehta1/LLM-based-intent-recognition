use std::env;
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4_O;
use crate::handlers::{joke_prompt, save_message, load_history, Message};

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
    let mut messages = vec![joke_prompt()];
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

    let request = ChatCompletionRequest::new(
        GPT4_O.to_string(),
        messages
    );

    let result = client.chat_completion(request).await.ok()?;

    // Extract and save the response
    if let Some(response_content) = result.choices[0].message.content.clone() {
        save_message("assistant", &response_content);
        return Some(response_content);
    }

    None
}