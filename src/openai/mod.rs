use serde::{Deserialize, Serialize};
use reqwest::{Error, Response, RequestBuilder};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OpenAIRequest {
    OpenAICompletionsRequest(OpenAICompletionsRequest),
    None
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OpenAIResponse {
    OpenAICompletionsResponse(OpenAICompletionsResponse),
    None,
}

#[derive(Debug, Clone)]
pub struct OpenAIHandler {
    pub headers: HeaderMap,
    pub request: OpenAIRequest,
    pub response: OpenAIResponse,
}

impl OpenAIHandler {
    pub fn new() -> OpenAIHandler {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE,HeaderValue::from_static("application/json"));
        OpenAIHandler {
            headers: headers,
            request: OpenAIRequest::None,
            response: OpenAIResponse::None,
        }
    }

    pub fn new_with_token(token: String) -> OpenAIHandler {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE,HeaderValue::from_static("application/json"));
        headers.insert("Authorization", HeaderValue::from_str(&token).expect(""));
        OpenAIHandler {
            headers: headers,
            request: OpenAIRequest::None,
            response: OpenAIResponse::None,
        }
    }

    pub fn set_token(&mut self, token: String) -> OpenAIHandler {
        self.headers.insert("Authorization", HeaderValue::from_str(&token).expect(""));
        self.clone()
    }

    pub fn headers(&mut self) -> HeaderMap {
        self.headers.to_owned()
    }

    pub async fn process(&mut self) -> Result<OpenAIResponse, Error> {
        let response = match self.post().await {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
	    match response.status().as_str() {
	        "200" => {
				info!("Successful Request");
	            let response_body = response.text().await?;
	            let openai_response: OpenAIResponse = match serde_json::from_str(&response_body) {
	                Ok(x) => x,
	                Err(error) => {
						error!("Error formatting response body: {:#?}", error);
						std::process::exit(1)
					}
	            };
                match &openai_response {
                    OpenAIResponse::OpenAICompletionsResponse(res) => {
                        res.clone().print_choices();
                    }
                    OpenAIResponse::None => {}
                }
                Ok(openai_response)
	        },
	        "400" => {
	            info!("Bad Request: {:?}", &response);
                self.process_error(response)
	        },
	        "401" => {
	            info!("Unauthorized Token: {:?}", &response);
                self.process_error(response)
	        },
	        _ => {
	            info!("Request Error: {:?}", &response);
                self.process_error(response)
	        },
	    }
    }

    fn process_error(&mut self, response: Response) -> Result<OpenAIResponse, Error> {
        match response.error_for_status() {
            Ok(error) => {
                warn!("Request Error: {:#?}", error);
                Ok(OpenAIResponse::None)
            },
            Err(error) => {
                warn!("Request Error: {:#?}", &error);
                Err(error)
            }
        }
    }

    async fn post(&mut self) -> Result<Response, Error> {
        let mut endpoint = String::from("https://api.openai.com");
        match &self.request {
            OpenAIRequest::OpenAICompletionsRequest(req) => {
                endpoint.push_str("/v1/completions");
            }
            OpenAIRequest::None => {}
        }
        debug!("Request Url: {:#?}", endpoint);
        debug!("Request Headers: {:#?}", self.headers());
	    let client = reqwest::Client::new();
	    client
	        .post(endpoint)
	        .headers(self.headers())
	        .json(&self.request)
            .send().await
    }

    pub fn request(&self) -> &OpenAIRequest {
        &self.request
    }

    pub fn set_request(&mut self, request: OpenAIRequest) {
        self.request = request;
    }

    pub fn response(&self) -> &OpenAIResponse {
        &self.response
    }

    pub fn set_response(&mut self, response: OpenAIResponse) {
        self.response = response;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAICompletionsRequest {
    pub model: String,
    pub prompt: String,
    pub max_tokens: usize,
    pub temperature: f32,
    pub user: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct OpenAICompletionsResponse {
    choices: Vec<Choice>,
}

impl OpenAICompletionsResponse {
	fn print_choices(self) {
		trace!("print choices");
		let choices_count = self.choices.len();
		for choice in self.choices {
			if choices_count == 1 {
				println!("{}",choice.remove_newline_prepend());
			} else {
                println!("OpenAI Response: {}",choice.remove_newline_prepend());
			}
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Choice {
    text: String,
}

impl Choice {
	fn remove_newline_prepend(self) -> String {
		trace!("remove newline prepend");
		self.text.clone().replacen("\n\n", "", 1)
	}
}
