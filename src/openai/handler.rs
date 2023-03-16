use reqwest::{Error, Response};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::multipart::Part;
use std::io::Read;
use std::io::BufReader;

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
                self.process_error(response).await
	        },
	        "401" => {
	            info!("Unauthorized Token: {:?}", &response);
                self.process_error(response).await
	        },
	        _ => {
	            info!("Request Error: {:?}", &response);
                self.process_error(response).await
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
            },
            OpenAIRequest::OpenAIFilesRequest(request) => {
                self.response = request.to_owned().process_response(response_body);
            },
            OpenAIRequest::OpenAIFileDeleteRequest(request) => {
                self.response = request.to_owned().process_response(response_body);
            },
            OpenAIRequest::OpenAIFileUploadRequest(request) => {
                self.response = request.to_owned().process_response(response_body);
            },
            OpenAIRequest::OpenAIFineTunesRequest(request) => {
                self.response = request.to_owned().process_response(response_body);
            },
            OpenAIRequest::OpenAIFineTuneCreateRequest(request) => {
                self.response = request.to_owned().process_response(response_body);
            },
            OpenAIRequest::OpenAIFineTuneCancelRequest(request) => {
                self.response = request.to_owned().process_response(response_body);
            },
            OpenAIRequest::OpenAIFineTuneEventsRequest(request) => {
                self.response = request.to_owned().process_response(response_body);
            },
            OpenAIRequest::OpenAIFineTuneDetailRequest(request) => {
                self.response = request.to_owned().process_response(response_body);
            },
            OpenAIRequest::OpenAIModelsRequest(request) => {
                self.response = request.to_owned().process_response(response_body);
            },
            OpenAIRequest::OpenAIModelDeleteRequest(request) => {
                self.response = request.to_owned().process_response(response_body);
            },
            OpenAIRequest::None => {},
        }

        Ok(self.response.clone())
    }

    async fn process_error(&mut self, response: Response) -> Result<OpenAIResponse, Error> {
        debug!("Request Error: {:#?}", response.text().await);
        std::process::exit(1)
        // match response.error_for_status() {
        //     Ok(error) => {
        //         warn!("Request Error: {:#?}", error);
        //         debug!("Bad Request Message: {:#?}", error.text().await);
        //         Ok(OpenAIResponse::None)
        //     },
        //     Err(error) => {
        //         warn!("Request Error: {:#?}", error);
        //         Err(error)
        //     }
        // }
    }

    pub fn endpoint(&mut self) -> String {
        let mut endpoint = String::from("https://api.openai.com");
        match &self.request {
            OpenAIRequest::OpenAICompletionsRequest(_) => {
                endpoint.push_str("/v1/completions");
            },
            OpenAIRequest::OpenAIFilesRequest(_) => {
                endpoint.push_str("/v1/files");
            },
            OpenAIRequest::OpenAIFileDeleteRequest(request) => {
                endpoint.push_str("/v1/files/");
            },
            OpenAIRequest::OpenAIFileUploadRequest(_) => {
                endpoint.push_str("/v1/files");
            },
            OpenAIRequest::OpenAIFineTunesRequest(_) => {
                endpoint.push_str("/v1/fine-tunes");
            },
            OpenAIRequest::OpenAIFineTuneCreateRequest(_) => {
                endpoint.push_str("/v1/fine-tunes");
            },
            OpenAIRequest::OpenAIFineTuneCancelRequest(_) => {
                endpoint.push_str("/v1/fine-tunes/");
            },
            OpenAIRequest::OpenAIFineTuneEventsRequest(_) => {
                endpoint.push_str("/v1/fine-tunes/");
            },
            OpenAIRequest::OpenAIFineTuneDetailRequest(_) => {
                endpoint.push_str("/v1/fine-tunes/");
            },
            OpenAIRequest::OpenAIModelsRequest(_) => {
                endpoint.push_str("/v1/models");
            },
            OpenAIRequest::OpenAIModelDeleteRequest(_) => {
                endpoint.push_str("/v1/models/");
            }
            OpenAIRequest::None => {

            },
        }
        endpoint
    }

    async fn post(&mut self) -> Result<Response, Error> {
        let endpoint = self.endpoint();
	    let client = reqwest::Client::new();
        match &self.request {
            OpenAIRequest::OpenAICompletionsRequest(request) => {
                debug!("Request being made with parameters: {:#?}", request);
        	    client.post(endpoint).headers(self.clone().headers()).json(request).send().await
            },
            OpenAIRequest::OpenAIFilesRequest(_) => {
        	    client.get(endpoint).headers(self.clone().headers()).send().await
            },
            OpenAIRequest::OpenAIFileDeleteRequest(request) => {
        	    client.delete(format!("{}{}", endpoint, request.filename)).headers(self.clone().headers()).send().await
            },
            OpenAIRequest::OpenAIFileUploadRequest(request) => {
                // async open file
                let file = match tokio::fs::File::open(request.file.to_path_buf()).await {
                    Ok(content) => content,
                    Err(error) => {
                        warn!("Error opening file: {:#?}", error);
                        std::process::exit(1)
                    }
                };
                let mut reader = BufReader::new(file.into_std().await);
                let mut buffer = Vec::new();
                reader.read_to_end(&mut buffer).unwrap();

                // create form
                let filename = String::from(request.file.file_name().unwrap().to_str().unwrap());
                let purpose = String::from(&request.purpose);
                let part = Part::bytes(buffer).file_name(filename);
                let form = reqwest::multipart::Form::new().part("file", part.into()).text("purpose", purpose);

        	    client.post(endpoint).headers(self.clone().headers()).multipart(form).send().await
            },
            OpenAIRequest::OpenAIFineTunesRequest(_) => {
        	    client.get(endpoint).headers(self.clone().headers()).send().await
            },
            OpenAIRequest::OpenAIFineTuneCreateRequest(request) => {
                debug!("Request being made with parameters: {:#?}", request);
        	    client.post(endpoint).headers(self.clone().headers()).json(request).send().await
            },
            OpenAIRequest::OpenAIFineTuneCancelRequest(request) => {
        	    client.post(format!("{}{}/cancel", endpoint, request.fine_tune_id)).headers(self.clone().headers()).send().await
            },
            OpenAIRequest::OpenAIFineTuneEventsRequest(request) => {
        	    client.get(format!("{}{}/events", endpoint, request.model_name)).headers(self.clone().headers()).send().await
            },
            OpenAIRequest::OpenAIModelsRequest(_) => {
        	    client.get(endpoint).headers(self.clone().headers()).send().await
            },
            OpenAIRequest::OpenAIModelDeleteRequest(request) => {
        	    client.delete(format!("{}{}", endpoint, request.model_name)).headers(self.clone().headers()).send().await
            }
            OpenAIRequest::None => {
                std::process::exit(1)
            },
            OpenAIRequest::OpenAIFineTuneDetailRequest(request) => {
        	    client.get(format!("{}{}", endpoint, request.fine_tune_id)).headers(self.clone().headers()).send().await
            },
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
