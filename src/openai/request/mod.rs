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
pub enum OpenAIRequest {
    OpenAICompletionsRequest(OpenAICompletionsRequest),
    OpenAIFilesRequest(OpenAIFilesRequest),
    OpenAIFileDeleteRequest(OpenAIFileDeleteRequest),
    OpenAIFileUploadRequest(OpenAIFileUploadRequest),
    OpenAIFineTunesRequest(OpenAIFineTunesRequest),
    OpenAIFineTuneCreateRequest(OpenAIFineTuneCreateRequest),
    OpenAIFineTuneCancelRequest(OpenAIFineTuneCancelRequest),
    OpenAIFineTuneDetailRequest(OpenAIFineTuneDetailRequest),
    OpenAIFineTuneEventsRequest(OpenAIFineTuneEventsRequest),
    OpenAIModelsRequest(OpenAIModelsRequest),
    OpenAIModelDeleteRequest(OpenAIModelDeleteRequest),
    None
}
