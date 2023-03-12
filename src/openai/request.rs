use serde::{Deserialize, Serialize};
use crate::openai::response::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OpenAIRequest {
    OpenAICompletionsRequest(OpenAICompletionsRequest),
    None
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAICompletionsRequest {
    pub model: String,
    pub prompt: String,
    pub max_tokens: usize,
    pub temperature: f32,
    pub user: String,
}

impl OpenAICompletionsRequest {
    pub fn process_response(self, response_body: String) -> OpenAIResponse {
            let chat_response: OpenAICompletionsResponse = match serde_json::from_str(&response_body) {
                Ok(res) => {
                    res
                },
                Err(error) => {
                    error!("Error formatting response body: {:#?}", error);
                    std::process::exit(1)
                }
            };
            OpenAIResponse::OpenAICompletionsResponse(chat_response)
    }
}
