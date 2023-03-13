use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIFileUploadResponse {
  id: String,
  object: String,
  bytes: u32,
  created_at: u32,
  filename: String,
  purpose: String,
}

impl OpenAIFileUploadResponse {
	pub fn print_file(self) {
		trace!("print file");
		println!("{} ({}) - {} bytes",self.id, self.filename, self.bytes);
	}
}
