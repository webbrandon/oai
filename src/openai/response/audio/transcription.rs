use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIAudioTranscriptionResponse {
  text: String,
}

impl OpenAIAudioTranscriptionResponse {
	pub fn print_response(self) {
		trace!("print response");
		println!("{}", self.text);
	}
}
