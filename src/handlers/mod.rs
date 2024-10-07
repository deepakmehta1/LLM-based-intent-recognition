use std::env;
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4_O;

pub async fn get_response(input: &str) -> Option<String> {
    let openai_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let client = OpenAIClient::new(openai_key);

    let request = ChatCompletionRequest::new(
        GPT4_O.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(input.to_string()),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }],
    );

    // Use `.ok()?` to convert the result to `Option` and `?` to unwrap it or return `None`
    let result = client.chat_completion(request).await.ok()?;
    // Extract content properly as Option<String>
    result.choices[0].message.content.clone()
}