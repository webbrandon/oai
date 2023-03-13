use serde::{Deserialize, Serialize};
use crate::openai::response::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIModelsRequest {
}

impl OpenAIModelsRequest {
    pub fn process_response(self, response_body: String) -> OpenAIResponse {
        debug!("Formatting response to type OpenAIModelsResponse: {:#?}", response_body);
        let response: OpenAIModelsResponse = match serde_json::from_str(&response_body) {
            Ok(res) => {
                res
            },
            Err(error) => {
                error!("Error formatting response body: {:#?}", error);
                std::process::exit(1)
            }
        };
        OpenAIResponse::OpenAIModelsResponse(response)
    }
}
