use serde::{Deserialize, Serialize};
use crate::openai::response::finetune::OpenAIFineTuneEvent;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIFineTuneDetailResponse {
    id: String,
    object: String,
    updated_at: u32,
    created_at: u32,
    model: String,
    organization_id: String,
    status: String,
    events: Vec<OpenAIFineTuneEvent>,
    fine_tuned_model: String,
  // "hyperparams": {
  //   "batch_size": 4,
  //   "learning_rate_multiplier": 0.1,
  //   "n_epochs": 4,
  //   "prompt_loss_weight": 0.1,
  // },
  // "result_files": [
  //   {
  //     "id": "file-QQm6ZpqdNwAaVC3aSz5sWwLT",
  //     "object": "file",
  //     "bytes": 81509,
  //     "created_at": 1614807863,
  //     "filename": "compiled_results.csv",
  //     "purpose": "fine-tune-results"
  //   }
  // ],
  // "validation_files": [],
  // "training_files": [
  //   {
  //     "id": "file-XGinujblHPwGLSztz8cPS8XY",
  //     "object": "file",
  //     "bytes": 1547276,
  //     "created_at": 1610062281,
  //     "filename": "my-data-train.jsonl",
  //     "purpose": "fine-tune-train"
  //   }
  // ],
}

impl OpenAIFineTuneDetailResponse {
	pub fn print_details(self) {
		trace!("print response");
		println!("{}:{}\nStatus ( {} )\nCreated: {}\nUpdated: {}\nFine Tune Model: {}",
            self.model,
            self.id,
            self.status,
            self.created_at,
            self.updated_at,
            self.fine_tuned_model,
        );
	}
}
