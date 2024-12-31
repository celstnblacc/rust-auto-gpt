use serde::{Deserialize, Serialize};

// https://platform.openai.com/docs/api-reference/making-requests
// Request payload
#[derive(Debug, Serialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ChatCompletion {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32,
}

// Response payload
#[derive(Debug, Deserialize)]
pub struct APIResponse {
    pub choices: Vec<APIChoice>,
    pub usage: Option<Usage>, // Optional if you don't need it
}

#[derive(Debug, Deserialize)]
pub struct APIMessage {
    pub content: String,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct APIChoice {
    pub message: APIMessage, // Correctly matched to the API response
    pub finish_reason: Option<String>,
    pub logprobs: Option<serde_json::Value>, // Generic type if you don't use this
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}
