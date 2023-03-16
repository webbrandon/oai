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
                CliRequest::CliModels(request_settings) => {
                    match request_settings.delete {
                        Some(model_name) => {
                            openai_handler.set_request(OpenAIRequest::OpenAIModelDeleteRequest(OpenAIModelDeleteRequest { model_name }));
                        },
                        None => {
                            openai_handler.set_request(OpenAIRequest::OpenAIModelsRequest(OpenAIModelsRequest {}));
                        },
                    }
                },
                CliRequest::CliFineTune(request_settings) => {
                    match &request_settings.training_file() {
                        Some(file) => {
                            openai_handler.set_request(OpenAIRequest::OpenAIFineTuneCreateRequest(OpenAIFineTuneCreateRequest {
                                suffix: request_settings.suffix().to_owned(),
                                compute_classification_metrics: request_settings.compute_classification_metrics().to_owned(),
                                prompt_loss_weight: request_settings.prompt_loss_weight().to_owned(),
                                n_epochs: request_settings.n_epochs().to_owned(),
                                model: request_settings.model().to_owned(),
                                validation_file: request_settings.validation_file().to_owned(),
                                training_file: file.to_owned(),
                                batch_size: request_settings.batch_size().to_owned(),
                                classification_n_classes: request_settings.classification_n_classes().to_owned(),
                                classification_positive_class: request_settings.classification_positive_class().to_owned(),
                                classification_betas: request_settings.classification_betas().to_owned(),
                            }));
                        },
                        None => {
                            match &request_settings.fine_tune_id() {
                                Some(fine_tune_id) => {
                                    if request_settings.clone().events().to_owned() {
                                        openai_handler.set_request(OpenAIRequest::OpenAIFineTuneEventsRequest(OpenAIFineTuneEventsRequest {
                                            model_name: fine_tune_id.to_owned(),
                                        }));
                                    } else if request_settings.clone().cancel() {
                                        openai_handler.set_request(OpenAIRequest::OpenAIFineTuneCancelRequest(OpenAIFineTuneCancelRequest {
                                            fine_tune_id: fine_tune_id.to_owned()
                                        }));
                                    } else {
                                        openai_handler.set_request(OpenAIRequest::OpenAIFineTuneDetailRequest(OpenAIFineTuneDetailRequest {
                                            fine_tune_id: fine_tune_id.to_owned(),
                                        }));
                                    }
                                },
                                None => {
                                    openai_handler.set_request(OpenAIRequest::OpenAIFineTunesRequest(OpenAIFineTunesRequest {}));
                                }
                            }
                        }
                    }
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
                logit_bias: cli_options.logit_bias().to_owned(),
                best_of: cli_options.best_of().to_owned(),
                frequency_penalty: cli_options.frequency_penalty().to_owned(),
                presence_penalty: cli_options.presence_penalty().to_owned(),
                stop: cli_options.stop().to_owned(),
                echo: cli_options.echo().to_owned(),
                logprobs: cli_options.logprobs().to_owned(),
                stream: cli_options.stream().to_owned(),
                n: cli_options.n().to_owned(),
                top_p: cli_options.top_p().to_owned(),
                suffix: cli_options.suffix().to_owned(),
            }));
        },
    }

    let openai_response = openai_handler.process().await;

    match openai_response {
        Ok(response) => {
            match response {
                OpenAIResponse::OpenAICompletionsResponse(data) => {
                    data.print_choices();
                },
                OpenAIResponse::OpenAIFilesResponse(data) => {
                    data.print_files()
                },
                OpenAIResponse::OpenAIFileDeleteResponse(data) => {
                    data.print_response()
                },
                OpenAIResponse::OpenAIFileUploadResponse(data) => {
                    data.print_file()
                },
                OpenAIResponse::OpenAIFineTunesResponse(data) => {
                    data.print_tunes()
                },
                OpenAIResponse::OpenAIFineTuneCreateResponse(data) => {
                    data.print_tune()
                },
                OpenAIResponse::OpenAIFineTuneCancelResponse(data) => {
                    data.print_response()
                },
                OpenAIResponse::OpenAIFineTuneEventsResponse(data) => {
                    data.print_events()
                },
                OpenAIResponse::OpenAIFineTuneDetailResponse(data) => {
                    data.print_details()
                },
                OpenAIResponse::OpenAIModelsResponse(data) => {
                    data.print_models()
                },
                OpenAIResponse::OpenAIModelDeleteResponse(data) => {
                    data.print_model()
                },
                OpenAIResponse::None => {},
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
