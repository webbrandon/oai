use serde::{Deserialize, Serialize};
use crate::openai::response::*;

use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIAudioTranslationRequest {
    pub file: PathBuf,
    pub model: String,
    pub prompt: Option<String>,
    pub response_format: String,
    pub temperature: f32,
}

impl OpenAIAudioTranslationRequest {
    pub fn process_response(self, response_body: String) -> OpenAIResponse {
        debug!("Formatting response to type OpenAIAudioTranslationResponse: {:#?}", response_body);
        let response: OpenAIAudioTranslationResponse = match serde_json::from_str(&response_body) {
            Ok(res) => {
                res
            },
            Err(error) => {
                error!("Error formatting response body: {:#?}", error);
                std::process::exit(1)
            }
        };
        OpenAIResponse::OpenAIAudioTranslationResponse(response)
    }
}
