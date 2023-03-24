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
        let response = match self.process_request().await {
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
            OpenAIRequest::OpenAIAudioTranslationRequest(request) => {
                self.response = request.to_owned().process_response(response_body);
            },
            OpenAIRequest::OpenAIAudioTranscriptionRequest(request) => {
                self.response = request.to_owned().process_response(response_body);
            },
            OpenAIRequest::OpenAICompletionsRequest(request) => {
                self.response = request.to_owned().process_response(response_body);
            },
            OpenAIRequest::OpenAICompletionEditRequest(request) => {
                self.response = request.to_owned().process_response(response_body);
            },
            OpenAIRequest::OpenAIEmbeddingRequest(request) => {
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
            OpenAIRequest::OpenAIImagesRequest(request) => {
                self.response = request.to_owned().process_response(response_body);
            },
            OpenAIRequest::OpenAIImageEditRequest(request) => {
                self.response = request.to_owned().process_response(response_body);
            },
            OpenAIRequest::OpenAIImageVariationRequest(request) => {
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
            OpenAIRequest::OpenAIAudioTranslationRequest(_) => {
                endpoint.push_str("/v1/audio/translations");
            }
            OpenAIRequest::OpenAIAudioTranscriptionRequest(_) => {
                endpoint.push_str("/v1/audio/transcriptions");
            },
            OpenAIRequest::OpenAICompletionsRequest(_) => {
                endpoint.push_str("/v1/completions");
            },
            OpenAIRequest::OpenAICompletionEditRequest(_) => {
                endpoint.push_str("/v1/edits");
            },
            OpenAIRequest::OpenAIEmbeddingRequest(_) => {
                endpoint.push_str("/v1/embeddings");
            },
            OpenAIRequest::OpenAIFilesRequest(_) => {
                endpoint.push_str("/v1/files");
            },
            OpenAIRequest::OpenAIFileDeleteRequest(_) => {
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
            OpenAIRequest::OpenAIImagesRequest(_) => {
                endpoint.push_str("/v1/images/generations");
            },
            OpenAIRequest::OpenAIImageEditRequest(_) => {
                endpoint.push_str("/v1/images/edits");
            },
            OpenAIRequest::OpenAIImageVariationRequest(_) => {
                endpoint.push_str("/v1/images/variations");
            },
            OpenAIRequest::OpenAIModelsRequest(_) => {
                endpoint.push_str("/v1/models");
            },
            OpenAIRequest::OpenAIModelDeleteRequest(_) => {
                endpoint.push_str("/v1/models/");
            },
            OpenAIRequest::None => {

            },
        }
        endpoint
    }

    async fn process_request(&mut self) -> Result<Response, Error> {
        let endpoint = self.endpoint();
	    let client = reqwest::Client::new();
        match &self.request {
            OpenAIRequest::OpenAIAudioTranslationRequest(request) => {
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

                let filename = String::from(request.file.file_name().unwrap().to_str().unwrap());
                let part = Part::bytes(buffer).file_name(filename);
                let mut form = reqwest::multipart::Form::new().part("file", part.into());

                match &request.prompt {
                    Some(prompt) => {
                        form = form.text("prompt", prompt.to_owned());
                    },
                    None => {},
                }
                form = form.text("model", request.model.to_owned());
                form = form.text("response_format", request.response_format.to_owned());
                form = form.text("temperature", request.temperature.to_owned().to_string());
                //debug!("Request bing made is: {:#?} ", client.clone().post(endpoint.clone()).headers(self.clone().headers().clone()).multipart(form));
        	    client.post(endpoint).headers(self.clone().headers()).multipart(form).send().await
            },
            OpenAIRequest::OpenAIAudioTranscriptionRequest(request) => {
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

                let filename = String::from(request.file.file_name().unwrap().to_str().unwrap());
                let part = Part::bytes(buffer).file_name(filename);
                let mut form = reqwest::multipart::Form::new().part("file", part.into());

                match &request.language {
                    Some(language) => {
                        form = form.text("temperature", language.to_owned());
                    }
                    None => {}
                }
                match &request.prompt {
                    Some(prompt) => {
                        form = form.text("prompt", prompt.to_owned());
                    },
                    None => {},
                }
                form = form.text("model", request.model.to_owned());
                form = form.text("response_format", request.response_format.to_owned());
                form = form.text("temperature", request.temperature.to_owned().to_string());

        	    client.post(endpoint).headers(self.clone().headers()).multipart(form).send().await
            },
            OpenAIRequest::OpenAICompletionsRequest(request) => {
                debug!("Request being made with parameters: {:#?}", request);
                self.headers.insert(CONTENT_TYPE,HeaderValue::from_static("application/json"));
        	    client.post(endpoint).headers(self.clone().headers()).json(request).send().await
            },
            OpenAIRequest::OpenAICompletionEditRequest(request) => {
                debug!("Request being made with parameters: {:#?}", request);
                self.headers.insert(CONTENT_TYPE,HeaderValue::from_static("application/json"));
        	    client.post(endpoint).headers(self.clone().headers()).json(request).send().await
            },
            OpenAIRequest::OpenAIEmbeddingRequest(request) => {
                debug!("Request being made with parameters: {:#?}", request);
                self.headers.insert(CONTENT_TYPE,HeaderValue::from_static("application/json"));
        	    client.post(endpoint).headers(self.clone().headers()).json(request).send().await
            },
            OpenAIRequest::OpenAIFilesRequest(_) => {
                self.headers.insert(CONTENT_TYPE,HeaderValue::from_static("application/json"));
        	    client.get(endpoint).headers(self.clone().headers()).send().await
            },
            OpenAIRequest::OpenAIFileDeleteRequest(request) => {
                self.headers.insert(CONTENT_TYPE,HeaderValue::from_static("application/json"));
        	    client.delete(format!("{}{}", endpoint, request.filename)).headers(self.clone().headers()).send().await
            },
            OpenAIRequest::OpenAIFileUploadRequest(request) => {
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

                let filename = String::from(request.file.file_name().unwrap().to_str().unwrap());
                let purpose = String::from(&request.purpose);
                let part = Part::bytes(buffer).file_name(filename);
                let form = reqwest::multipart::Form::new().part("file", part.into()).text("purpose", purpose);

        	    client.post(endpoint).headers(self.clone().headers()).multipart(form).send().await
            },
            OpenAIRequest::OpenAIFineTunesRequest(_) => {
                self.headers.insert(CONTENT_TYPE,HeaderValue::from_static("application/json"));
        	    client.get(endpoint).headers(self.clone().headers()).send().await
            },
            OpenAIRequest::OpenAIFineTuneCreateRequest(request) => {
                debug!("Request being made with parameters: {:#?}", request);
                self.headers.insert(CONTENT_TYPE,HeaderValue::from_static("application/json"));
        	    client.post(endpoint).headers(self.clone().headers()).json(request).send().await
            },
            OpenAIRequest::OpenAIFineTuneCancelRequest(request) => {
                self.headers.insert(CONTENT_TYPE,HeaderValue::from_static("application/json"));
        	    client.post(format!("{}{}/cancel", endpoint, request.fine_tune_id)).headers(self.clone().headers()).send().await
            },
            OpenAIRequest::OpenAIFineTuneEventsRequest(request) => {
                self.headers.insert(CONTENT_TYPE,HeaderValue::from_static("application/json"));
        	    client.get(format!("{}{}/events", endpoint, request.fine_tune_id)).headers(self.clone().headers()).send().await
            },
            OpenAIRequest::OpenAIFineTuneDetailRequest(request) => {
                self.headers.insert(CONTENT_TYPE,HeaderValue::from_static("application/json"));
        	    client.get(format!("{}{}", endpoint, request.fine_tune_id)).headers(self.clone().headers()).send().await
            },
            OpenAIRequest::OpenAIImagesRequest(request) => {
                debug!("Request being made with parameters: {:#?}", request);
                self.headers.insert(CONTENT_TYPE,HeaderValue::from_static("application/json"));
        	    client.post(endpoint).headers(self.clone().headers()).json(request).send().await
            },
            OpenAIRequest::OpenAIImageEditRequest(request) => {
                let image_file = request.image.as_ref().unwrap();
                let img = match tokio::fs::File::open(&image_file).await {
                    Ok(content) => content,
                    Err(error) => {
                        warn!("Error opening file: {:#?}", error);
                        std::process::exit(1)
                    }
                };
                let user = request.user.as_ref().unwrap();
                let mut img_reader = BufReader::new(img.into_std().await);
                let mut img_src = Vec::new();
                img_reader.read_to_end(&mut img_src).unwrap();

                match &request.mask {
                    Some(mask_file) => {
                            let img_filename = String::from(image_file.file_name().unwrap().to_str().unwrap());
                            let img_part = Part::bytes(img_src).file_name(img_filename);

                            let mask = match tokio::fs::File::open(&mask_file).await {
                                Ok(content) => content,
                                Err(error) => {
                                    warn!("Error opening file: {:#?}", error);
                                    std::process::exit(1)
                                }
                            };
                            let mut mask_reader = BufReader::new(mask.into_std().await);
                            let mut mask_src = Vec::new();
                            mask_reader.read_to_end(&mut mask_src).unwrap();
                            let mask_filename = String::from(mask_file.file_name().unwrap().to_str().unwrap());
                            let mask_part = Part::bytes(mask_src).file_name(mask_filename);
                            let form = reqwest::multipart::Form::new()
                                .part("image", img_part.into())
                                .part("mask", mask_part.into());
                            let form = form.text("n", request.clone().n.to_string())
                                .text("size", request.clone().size)
                                .text("response_format", request.clone().response_format)
                                .text("prompt", request.clone().prompt.unwrap())
                                .text("user", user.clone());

                    	    client.post(endpoint).headers(self.clone().headers()).multipart(form).send().await
                    }
                    None => {
                        let img_filename = String::from(image_file.file_name().unwrap().to_str().unwrap());
                        let img_part = Part::bytes(img_src).file_name(img_filename);

                        let form = reqwest::multipart::Form::new().part("image", img_part.into());
                        let form = form.text("n", request.clone().n.to_string())
                            .text("size", request.clone().size)
                            .text("response_format", request.clone().response_format)
                            .text("prompt", request.clone().prompt.unwrap())
                            .text("user", user.clone());

                        client.post(endpoint).headers(self.clone().headers()).multipart(form).send().await
                    }
                }
            },
            OpenAIRequest::OpenAIImageVariationRequest(request) => {
                let user = request.user.as_ref().unwrap();
                let image_file = request.image.as_ref().unwrap();
                let img = match tokio::fs::File::open(&image_file).await {
                    Ok(content) => content,
                    Err(error) => {
                        warn!("Error opening file: {:#?}", error);
                        std::process::exit(1)
                    }
                };
                let mut img_reader = BufReader::new(img.into_std().await);
                let mut img_src = Vec::new();
                img_reader.read_to_end(&mut img_src).unwrap();

                let img_filename = String::from(image_file.file_name().unwrap().to_str().unwrap());
                let img_part = Part::bytes(img_src).file_name(img_filename);

                let form = reqwest::multipart::Form::new().part("image", img_part.into());
                let form = form.text("n", request.clone().n.to_string())
                    .text("size", request.clone().size)
                    .text("response_format", request.clone().response_format)
                    .text("user", user.clone());

                client.post(endpoint).headers(self.clone().headers()).multipart(form).send().await
            },
            OpenAIRequest::OpenAIModelsRequest(_) => {
                self.headers.insert(CONTENT_TYPE,HeaderValue::from_static("application/json"));
        	    client.get(endpoint).headers(self.clone().headers()).send().await
            },
            OpenAIRequest::OpenAIModelDeleteRequest(request) => {
                self.headers.insert(CONTENT_TYPE,HeaderValue::from_static("application/json"));
        	    client.delete(format!("{}{}", endpoint, request.model_name)).headers(self.clone().headers()).send().await
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
