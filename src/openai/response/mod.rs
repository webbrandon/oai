use serde::{Deserialize, Serialize};

pub mod completions;
pub mod files;
pub mod models;
pub mod finetune;
pub mod audio;
pub mod images;
pub mod embeddings;

pub use completions::*;
pub use files::*;
pub use models::*;
pub use finetune::*;
pub use audio::*;
pub use images::*;
pub use embeddings::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OpenAIResponse {
    OpenAIAudioTranslationResponse(OpenAIAudioTranslationResponse),
    OpenAIAudioTranscriptionResponse(OpenAIAudioTranscriptionResponse),
    OpenAICompletionsResponse(OpenAICompletionsResponse),
    OpenAICompletionEditResponse(OpenAICompletionEditResponse),
    OpenAIEmbeddingResponse(OpenAIEmbeddingResponse),
    OpenAIFilesResponse(OpenAIFilesResponse),
    OpenAIFileDeleteResponse(OpenAIFileDeleteResponse),
    OpenAIFileUploadResponse(OpenAIFileUploadResponse),
    OpenAIFineTunesResponse(OpenAIFineTunesResponse),
    OpenAIFineTuneCreateResponse(OpenAIFineTuneCreateResponse),
    OpenAIFineTuneCancelResponse(OpenAIFineTuneCancelResponse),
    OpenAIFineTuneDetailResponse(OpenAIFineTuneDetailResponse),
    OpenAIFineTuneEventsResponse(OpenAIFineTuneEventsResponse),
    OpenAIImagesResponse(OpenAIImagesResponse),
    OpenAIImageEditResponse(OpenAIImageEditResponse),
    OpenAIImageVariationResponse(OpenAIImageVariationResponse),
    OpenAIModelsResponse(OpenAIModelsResponse),
    OpenAIModelDeleteResponse(OpenAIModelDeleteResponse),
    None,
}
