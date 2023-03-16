use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIFineTuneCreateResponse {
    id: String,
    object: String,
    updated_at: u32,
    created_at: u32,
    model: String,
    organization_id: String,
    status: String,
    // TODO later
    // "events": [
    //   {
    //     "object": "fine-tune-event",
    //     "created_at": 1614807352,
    //     "level": "info",
    //     "message": "Job enqueued. Waiting for jobs ahead to complete. Queue number: 0."
    //   }
    // ],
    // "fine_tuned_model": null,
    // "hyperparams": {
    //   "batch_size": 4,
    //   "learning_rate_multiplier": 0.1,
    //   "n_epochs": 4,
    //   "prompt_loss_weight": 0.1,
    // },
    // "result_files": [],
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

impl OpenAIFineTuneCreateResponse {
	pub fn print_tune(self) {
		trace!("print file");
		println!("{}({}) - Model({}) - {}" , self.id, self.object, self.model, self.status);
	}
}
