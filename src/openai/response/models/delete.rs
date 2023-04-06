use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIModelDeleteResponse {
  id: String,
  object: String,
  deleted: bool,
}

impl OpenAIModelDeleteResponse {
	pub fn print_model(self) {
		trace!("print model");
		println!("{} - Deleted ({})" , self.id, self.deleted)
    }
}
