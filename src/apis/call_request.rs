use crate::models::general::llm::{APIResponse, ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use std::env;
use std::error::Error;

const MODEL_GPT: &str = "gpt-4-turbo-2024-04-09";

// Call large language model: gpt-4-turbo-2024-04-09
// dyn means dynamic dispatch at runtime
// + Send, mean the owner can be moved to another thread and shared. We want to call this function twice

// Helper function to simplify error wrapping
fn wrap_error<E: std::error::Error + Send + Sync + 'static>(
    err: E,
) -> Box<dyn std::error::Error + Send> {
    Box::new(std::io::Error::new(std::io::ErrorKind::Other, err))
}

// Call GPT function
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn Error + Send>> {
    dotenv().ok();

    // Extract API keys
    let api_key = env::var("OPEN_AI_KEY").map_err(wrap_error)?;
    let api_org = env::var("OPEN_AI_ORG").map_err(wrap_error)?;

    let url = "https://api.openai.com/v1/chat/completions";

    // Set headers
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key)).map_err(wrap_error)?,
    );
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(&api_org).map_err(wrap_error)?,
    );

    // Create client
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(wrap_error)?;

    // Prepare payload
    let chat_completion = ChatCompletion {
        model: MODEL_GPT.to_string(),
        messages,
        temperature: 0.1,
    };

    // Send request and deserialize response
    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(wrap_error)?
        .json()
        .await
        .map_err(wrap_error)?;

    // Extract and return the content of the first choice
    let response_content = res
        .choices
        .get(0)
        .map(|choice| choice.message.content.clone())
        .ok_or_else(|| {
            wrap_error(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No valid response content found",
            ))
        })?
        .trim()
        .to_string();
    Ok(response_content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_call_to_openai() {
        let message = Message {
            role: "user".to_string(),
            content: "Hello, this is a test. Give me a short response.".to_string(),
        };

        let messages = vec![message];
        match call_gpt(messages).await {
            Ok(res_str) => {
                dbg!(&res_str);
                assert!(!res_str.is_empty(), "Response should not be empty");
            }
            Err(err) => {
                eprintln!("Error occurred: {:?}", err);
                assert!(false, "API call failed");
            }
        }
    }
}
