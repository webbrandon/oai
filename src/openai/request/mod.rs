use serde::{Deserialize, Serialize};

pub mod completions;
pub mod files;
pub mod models;
pub mod finetune;
pub mod audio;
pub mod images;

pub use completions::*;
pub use files::*;
pub use models::*;
pub use finetune::*;
pub use audio::*;
pub use images::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OpenAIRequest {
    OpenAIAudioTranslationRequest(OpenAIAudioTranslationRequest),
    OpenAIAudioTranscriptionRequest(OpenAIAudioTranscriptionRequest),
    OpenAICompletionsRequest(OpenAICompletionsRequest),
    OpenAICompletionEditRequest(OpenAICompletionEditRequest),
    OpenAIFilesRequest(OpenAIFilesRequest),
    OpenAIFileDeleteRequest(OpenAIFileDeleteRequest),
    OpenAIFileUploadRequest(OpenAIFileUploadRequest),
    OpenAIFineTunesRequest(OpenAIFineTunesRequest),
    OpenAIFineTuneCreateRequest(OpenAIFineTuneCreateRequest),
    OpenAIFineTuneCancelRequest(OpenAIFineTuneCancelRequest),
    OpenAIFineTuneDetailRequest(OpenAIFineTuneDetailRequest),
    OpenAIFineTuneEventsRequest(OpenAIFineTuneEventsRequest),
    OpenAIImagesRequest(OpenAIImagesRequest),
    OpenAIImageEditRequest(OpenAIImageEditRequest),
    OpenAIImageVariationRequest(OpenAIImageVariationRequest),
    OpenAIModelsRequest(OpenAIModelsRequest),
    OpenAIModelDeleteRequest(OpenAIModelDeleteRequest),
    None
}
