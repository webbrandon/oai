use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIModelsResponse {
    data: Vec<OpenAIModel>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIModel {
  id: String,
  object: String,
  owned_by: String,
}

impl OpenAIModelsResponse {
	pub fn print_models(self) {
		trace!("print models");
		for model in self.data {
			println!("{}",model.id);
		}
	}
}
