use serde::{Deserialize, Serialize};
use crate::openai::response::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIFineTuneCreateRequest {
    pub training_file: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_file: Option<String>,
    pub model: String,
    pub n_epochs: i32,
    pub prompt_loss_weight: f32,
    pub compute_classification_metrics: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<u32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_n_classes: Option<u32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_positive_class: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_betas: Option<Vec<String>>
}

impl OpenAIFineTuneCreateRequest {
    pub fn process_response(self, response_body: String) -> OpenAIResponse {
        debug!("Formatting response to type OpenAIFineTuneCreateResponse: {:#?}", response_body);
        let response: OpenAIFineTuneCreateResponse = match serde_json::from_str(&response_body) {
            Ok(res) => {
                res
            },
            Err(error) => {
                error!("Error formatting response body: {:#?}", error);
                std::process::exit(1)
            }
        };
        OpenAIResponse::OpenAIFineTuneCreateResponse(response)
    }
}
