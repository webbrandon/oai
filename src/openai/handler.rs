use reqwest::{Error, Response};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use crate::openai::request::*;
use crate::openai::response::*;

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
        headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", token)).expect(""));
        OpenAIHandler {
            headers: headers,
            request: OpenAIRequest::None,
            response: OpenAIResponse::None,
        }
    }

    pub fn set_token(&mut self, token: String) -> OpenAIHandler {
        self.headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", token)).expect(""));
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
	            self.process_success(response).await
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

    async fn process_success(&mut self, response: Response) -> Result<OpenAIResponse, Error> {
        let response_body = match response.text().await {
            Ok(body) => body,
            Err(_) => String::new()
        };

        match &self.request {
            OpenAIRequest::OpenAICompletionsRequest(request) => {
                self.response = request.to_owned().process_response(response_body);
            }
            OpenAIRequest::None => {}
        }

        Ok(self.response.clone())
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

    pub fn endpoint(&mut self) -> String {
        let mut endpoint = String::from("https://api.openai.com");
        match &self.request {
            OpenAIRequest::OpenAICompletionsRequest(_) => {
                endpoint.push_str("/v1/completions");
            }
            OpenAIRequest::None => {

            }
        }
        endpoint
    }

    async fn post(&mut self) -> Result<Response, Error> {
        let endpoint = self.endpoint();
	    let client = reqwest::Client::new();
        match &self.request {
            OpenAIRequest::OpenAICompletionsRequest(request) => {
        	    client.post(endpoint).headers(self.clone().headers()).json(request).send().await
            }
            OpenAIRequest::None => {
                std::process::exit(1)
            }
        }
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
