use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIFilesResponse {
    data: Vec<OpenAIMFile>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIMFile {
  id: String,
  object: String,
  bytes: u32,
  created_at: u32,
  filename: String,
  purpose: String,
}

impl OpenAIFilesResponse {
	pub fn print_files(self) {
		trace!("print files");
		for file in &self.data {
			println!("{} ({}) - {} bytes",file.id, file.filename, file.bytes);
		}
        if self.data.is_empty() {
            debug!("No files belong to owner");
        }
	}
}
