use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIFineTuneEventsResponse {
    data: Vec<OpenAIFineTuneEvent>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIFineTuneEvent {
    created_at: u32,
    level: String,
    message: String,
}

impl OpenAIFineTuneEventsResponse {
	pub fn print_events(self) {
		trace!("print events");
		for event in &self.data {
			println!("{}::{}: {}" , event.level, event.created_at, event.message);
		}
        if self.data.is_empty() {
            debug!("No events belong to fine-tune");
        }
	}
}
