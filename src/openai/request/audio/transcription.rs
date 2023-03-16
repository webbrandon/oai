use serde::{Deserialize, Serialize};
use crate::openai::response::*;

use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIAudioTranscriptionRequest {
    pub file: PathBuf,
    pub model: String,
    pub prompt: Option<String>,
    pub response_format: String,
    pub temperature: f32,
    pub language: Option<String>,
}

impl OpenAIAudioTranscriptionRequest {
    pub fn process_response(self, response_body: String) -> OpenAIResponse {
        debug!("Formatting response to type OpenAIAudioTranscriptionResponse: {:#?}", response_body);
        let response: OpenAIAudioTranscriptionResponse = match serde_json::from_str(&response_body) {
            Ok(res) => {
                res
            },
            Err(error) => {
                error!("Error formatting response body: {:#?}", error);
                std::process::exit(1)
            }
        };
        OpenAIResponse::OpenAIAudioTranscriptionResponse(response)
    }
}
