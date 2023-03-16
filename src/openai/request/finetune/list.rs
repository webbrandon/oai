use serde::{Deserialize, Serialize};
use crate::openai::response::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIFineTunesRequest {
}

impl OpenAIFineTunesRequest {
    pub fn process_response(self, response_body: String) -> OpenAIResponse {
        debug!("Formatting response to type OpenAIFineTunesResponse: {:#?}", response_body);
        let response: OpenAIFineTunesResponse = match serde_json::from_str(&response_body) {
            Ok(res) => {
                res
            },
            Err(error) => {
                error!("Error formatting response body: {:#?}", error);
                std::process::exit(1)
            }
        };
        OpenAIResponse::OpenAIFineTunesResponse(response)
    }
}
