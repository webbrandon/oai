use serde::{Deserialize, Serialize};
use reqwest::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatGPTRequest {
    pub model: String,
    pub prompt: String,
    pub max_tokens: usize,
    pub temperature: f32,
    pub user: String,
}

impl ChatGPTRequest {
	fn create_headers(&self, token: &str) -> Result<reqwest::header::HeaderMap, Error> {
		trace!("create headers");
	    let mut headers = reqwest::header::HeaderMap::new();
	    headers.insert(
	        reqwest::header::CONTENT_TYPE,
	        reqwest::header::HeaderValue::from_static("application/json")
	    );
	    headers.insert(
	        "Authorization",
	        reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token))
	            .map_err(|e| e).unwrap(),
	    );
	    Ok(headers)
	}

	pub async fn converse(self, token: &str) -> Result<(), Error> {
		trace!("converse");
	    let endpoint = "https://api.openai.com/v1/completions";
	    let headers = match self.create_headers(&token) {
	        Ok(x) => x,
	        Err(e) => {
	            error!("Failed at creating headers: {:?}", e);
	            std::process::exit(1)
	        }
	    };

	    let client = reqwest::Client::new();
	    let response = client
	        .post(endpoint.clone())
	        .headers(headers.clone())
	        .json(&self)
	        .send().await?;

	    match response.status().as_str() {
	        "200" => {
				trace!("Success response from OpenAI");
	            let response_body = response.text().await?;
	            let chat_response: ChatGPTResponse = match serde_json::from_str(&response_body) {
	                Ok(x) => x,
	                Err(error) => {
						error!("Error formatting response body: {:#?}", error);
						std::process::exit(1)
					}
	            };
	            chat_response.print_choices();
	        },
	        "400" => {
	            debug!("Bad Request: {:?}", client
	                .post(endpoint.clone())
	                .headers(headers.clone())
	                .json(&self));
	        },
	        "401" => {
	            debug!("Unauthorized Token: {:?}", headers.clone());
	        },
	        _ => {
	            debug!("Request Error: {:?}", headers.clone());
	        },
	    }

	    Ok(())
	}
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatGPTResponse {
    choices: Vec<Choice>,
}

impl ChatGPTResponse {
	fn print_choices(self) {
		trace!("print choices");
		let choices_count = self.choices.len();
		for choice in self.choices {
			if choices_count == 1 {
				println!("{}",choice.remove_newline_prepend());
			} else {
                println!("ChatGBT Response: {}",choice.remove_newline_prepend());
			}
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
struct Choice {
    text: String,
}

impl Choice {
	fn remove_newline_prepend(self) -> String {
		trace!("remove newline prepend");
		self.text.clone().replacen("\n\n", "", 1)
	}
}
