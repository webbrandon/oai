use serde::{Deserialize, Serialize};

pub mod completions;
pub mod files;
pub mod models;
pub mod finetune;
pub mod audio;

pub use completions::*;
pub use files::*;
pub use models::*;
pub use finetune::*;
pub use audio::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OpenAIRequest {
    OpenAIAudioTranslationRequest(OpenAIAudioTranslationRequest),
    OpenAIAudioTranscriptionRequest(OpenAIAudioTranscriptionRequest),
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
