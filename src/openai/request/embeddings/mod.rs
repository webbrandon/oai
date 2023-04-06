use serde::{Deserialize, Serialize};
use crate::openai::response::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIEmbeddingRequest {
    pub model: String,
    pub input: String,
    pub user: String,
}

impl OpenAIEmbeddingRequest {
    pub fn process_response(self, response_body: String) -> OpenAIResponse {
            let chat_response: OpenAIEmbeddingResponse = match serde_json::from_str(&response_body) {
                Ok(res) => {
                    res
                },
                Err(error) => {
                    error!("Error formatting response body: {:#?}", error);
                    std::process::exit(1)
                }
            };
            OpenAIResponse::OpenAIEmbeddingResponse(chat_response)
    }
}
