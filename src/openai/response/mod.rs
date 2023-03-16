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
pub enum OpenAIResponse {
    OpenAIAudioTranslationResponse(OpenAIAudioTranslationResponse),
    OpenAIAudioTranscriptionResponse(OpenAIAudioTranscriptionResponse),
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
