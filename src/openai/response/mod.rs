use serde::{Deserialize, Serialize};

pub mod completions;
pub mod files;
pub mod models;
pub mod finetune;

pub use completions::*;
pub use files::*;
pub use models::*;
pub use finetune::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OpenAIResponse {
    OpenAICompletionsResponse(OpenAICompletionsResponse),
    OpenAIFilesResponse(OpenAIFilesResponse),
    OpenAIFileDeleteResponse(OpenAIFileDeleteResponse),
    OpenAIFileUploadResponse(OpenAIFileUploadResponse),
    OpenAIFineTunesResponse(OpenAIFineTunesResponse),
    OpenAIFineTuneCreateResponse(OpenAIFineTuneCreateResponse),
    OpenAIFineTuneCancelResponse(OpenAIFineTuneCancelResponse),
    OpenAIFineTuneDetailResponse(OpenAIFineTuneDetailResponse),
    OpenAIFineTuneEventsResponse(OpenAIFineTuneEventsResponse),
    OpenAIModelsResponse(OpenAIModelsResponse),
    OpenAIModelDeleteResponse(OpenAIModelDeleteResponse),
    None,
}
