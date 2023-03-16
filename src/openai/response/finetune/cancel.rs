use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIFineTuneCancelResponse {
    id: String,
    object: String,
    updated_at: u32,
    created_at: u32,
    model: String,
    organization_id: String,
    status: String,
}

impl OpenAIFineTuneCancelResponse {
	pub fn print_response(self) {
		trace!("print response");
		println!("{}({}) - Status ( {} )",self.id, self.model, self.status);
	}
}
