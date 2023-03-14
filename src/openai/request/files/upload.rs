use serde::{Deserialize, Serialize};
use crate::openai::response::*;

use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIFileUploadRequest {
    pub file: PathBuf,
    pub purpose: String,
}

impl OpenAIFileUploadRequest {
    pub fn process_response(self, response_body: String) -> OpenAIResponse {
        debug!("Formatting response to type OpenAIFileUploadResponse: {:#?}", response_body);
        let response: OpenAIFileUploadResponse = match serde_json::from_str(&response_body) {
            Ok(res) => {
                res
            },
            Err(error) => {
                error!("Error formatting response body: {:#?}", error);
                std::process::exit(1)
            }
        };
        OpenAIResponse::OpenAIFileUploadResponse(response)
    }
}
