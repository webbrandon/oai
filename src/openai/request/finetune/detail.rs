use serde::{Deserialize, Serialize};
use crate::openai::response::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIFineTuneDetailRequest {
    pub fine_tune_id: String,
}

impl OpenAIFineTuneDetailRequest {
    pub fn process_response(self, response_body: String) -> OpenAIResponse {
        debug!("Formatting response to type OpenAIFineTuneDetailResponse: {:#?}", response_body);
        let response: OpenAIFineTuneDetailResponse = match serde_json::from_str(&response_body) {
            Ok(res) => {
                res
            },
            Err(error) => {
                error!("Error formatting response body: {:#?}", error);
                std::process::exit(1)
            }
        };
        OpenAIResponse::OpenAIFineTuneDetailResponse(response)
    }
}
