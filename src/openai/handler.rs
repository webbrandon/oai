use reqwest::{Error, Response};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::{multipart, Body, Client};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};
use reqwest::multipart::Part;
use std::io;
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
            },
            OpenAIRequest::OpenAIFilesRequest(request) => {
                self.response = request.to_owned().process_response(response_body);
            },
            OpenAIRequest::OpenAIFileUploadRequest(request) => {
                self.response = request.to_owned().process_response(response_body);
            },
            OpenAIRequest::OpenAIModelsRequest(request) => {
                self.response = request.to_owned().process_response(response_body);
            },
            OpenAIRequest::None => {},
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
            },
            OpenAIRequest::OpenAIFilesRequest(_) => {
                endpoint.push_str("/v1/files");
            },
            OpenAIRequest::OpenAIFileUploadRequest(_) => {
                endpoint.push_str("/v1/files");
            },
            OpenAIRequest::OpenAIModelsRequest(_) => {
                endpoint.push_str("/v1/models");
            },
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
        	    client.post(endpoint).headers(self.clone().headers()).json(request).send().await
            },
            OpenAIRequest::OpenAIFilesRequest(_) => {
        	    client.get(endpoint).headers(self.clone().headers()).send().await
            },
            OpenAIRequest::OpenAIFileUploadRequest(request) => {
                // let file = match File::open(&request.file).await {
                //     Ok(content) => content,
                //     Err(error) => {
                //         warn!("Error opening file: {:#?}", error);
                //         std::process::exit(1)
                //     }
                // };
                let file = match tokio::fs::File::open(request.file.to_path_buf()).await {
                    Ok(content) => content,
                    Err(error) => {
                        warn!("Error opening file: {:#?}", error);
                        std::process::exit(1)
                    }
                };
                let mut reader = BufReader::new(file.into_std().await);
                let mut buffer = Vec::new();

                // Read file into vector.
                reader.read_to_end(&mut buffer).unwrap();

                // read file body stream
                // let stream = FramedRead::new(file, BytesCodec::new());
                // let file_body = Body::wrap_stream(stream);
                let filename = String::from(request.file.file_name().unwrap().to_str().unwrap());
                let purpose = String::from(&request.purpose);

                // let file = tokio::fs::File::open(path.as_ref())
                //     .await
                //     .map_err(|e| OpenAIError::FileReadError(e.to_string()))?;
                // let stream = FramedRead::new(file, BytesCodec::new());
                // let body = Body::wrap_stream(stream);

                // let file_part = reqwest::multipart::Part::stream(file_body)
                //     .file_name(filename)
                //     .mime_str("application/octet-stream")
                //     .unwrap();
                // let bytes = match Body::as_bytes(file_body) {
                //     Some(x) => x,
                //     None => std::process::exit(1)
                // };
                let part = Part::bytes(buffer).file_name(filename);
                //create the multipart form
                // let form = multipart::Form::new()
                //     .part("file", file_part)
                //     .text("purpose", purpose);

                let form = reqwest::multipart::Form::new().part("file", part.into()).text("purpose", purpose);
        	    match client.post(endpoint).headers(self.clone().headers()).multipart(form).send().await {
        	        Ok(x) => {
                        println!("{:#?}", x);
                        Ok(x)
                    }
        	        Err(_) => {std::process::exit(1)}
        	    }
            },
            OpenAIRequest::OpenAIModelsRequest(_) => {
        	    client.get(endpoint).headers(self.clone().headers()).send().await
            },
            OpenAIRequest::None => {
                std::process::exit(1)
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
