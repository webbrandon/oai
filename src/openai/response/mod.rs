use serde::{Deserialize, Serialize};

pub mod completions;
pub mod files;
pub mod models;

pub use completions::*;
pub use files::*;
pub use models::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OpenAIResponse {
    OpenAICompletionsResponse(OpenAICompletionsResponse),
    OpenAIFilesResponse(OpenAIFilesResponse),
    OpenAIFileUploadResponse(OpenAIFileUploadResponse),
    OpenAIModelsResponse(OpenAIModelsResponse),
    None,
}
