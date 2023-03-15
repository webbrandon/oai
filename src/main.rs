#[macro_use]
extern crate log;

mod openai;
mod cli;
mod cmdln;

use openai::*;
use cli::{CliInterface, CliRequest};
use structopt::StructOpt;
use reqwest::Error;
use std::env;

// Logging interfaces
use env_logger;
use log::LevelFilter;
use env_logger::Builder;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut cli_options = CliInterface::from_args();
    init_log(&cli_options.verbose);

    let mut openai_handler = OpenAIHandler::new_with_token(cli_options.clone().api_auth_token());
    match cli_options.args {
        Some(subcommand) => {
            match subcommand {
                CliRequest::CliFiles(request_settings) => {
                    match &request_settings.file() {
                        Some(file_path) => {
                            openai_handler.set_request(OpenAIRequest::OpenAIFileUploadRequest(OpenAIFileUploadRequest {
                                file: file_path.to_owned(),
                                purpose: request_settings.purpose().to_owned()
                            }));
                        },
                        None => {
                            match request_settings.delete() {
                                Some(filename) => {
                                    openai_handler.set_request(OpenAIRequest::OpenAIFileDeleteRequest(OpenAIFileDeleteRequest { filename }));
                                }
                                None => {
                                    openai_handler.set_request(OpenAIRequest::OpenAIFilesRequest(OpenAIFilesRequest {}));
                                }
                            }
                        }
                    }
                },
                CliRequest::CliModels(_) => {
                    openai_handler.set_request(OpenAIRequest::OpenAIModelsRequest(OpenAIModelsRequest {}));
                },
            }
        },
        None => {
            openai_handler.set_request(OpenAIRequest::OpenAICompletionsRequest(OpenAICompletionsRequest {
                model: cli_options.model().to_owned(),
                prompt: cli_options.prompt().to_owned(),
                max_tokens: cli_options.max_tokens().to_owned(),
                temperature: cli_options.temperature().to_owned(),
                user: cli_options.user().to_owned(),
            }));
        },
    }

    let openai_response = openai_handler.process().await;

    match openai_response {
        Ok(response) => {
            match response {
                openai::OpenAIResponse::OpenAICompletionsResponse(data) => {
                    data.print_choices();
                },
                openai::OpenAIResponse::OpenAIFilesResponse(data) => {
                    data.print_files()
                },
                openai::OpenAIResponse::OpenAIFileDeleteResponse(data) => {
                    data.print_response()
                },
                openai::OpenAIResponse::OpenAIFileUploadResponse(data) => {
                    data.print_file()
                },
                openai::OpenAIResponse::OpenAIModelsResponse(data) => {
                    data.print_models()
                },
                openai::OpenAIResponse::None => {},
            }
        }
        Err(_) => {}
    }

    Ok(())
}

fn init_log(is_verbose: &u8) {
    let environment_override: Option<u8> = match env::var("DEBUG") {
        Ok(value)  => {Some(value.trim().parse().expect("Wanted a number"))},
        Err(error) => {trace!("{}", error);None}
    };
    let is_verbose = match environment_override {Some(value)=>{value},None=>{is_verbose.to_owned()}};
    let logging_level = match is_verbose {
        0 => {LevelFilter::Off},
        1 => {LevelFilter::Info},
        2 => {LevelFilter::Warn},
        3 => {LevelFilter::Error},
        4 => {LevelFilter::Debug},
        5 => {LevelFilter::Trace},
        _ => {LevelFilter::max()},
    };
    Builder::new().filter_level(logging_level).init();
}
