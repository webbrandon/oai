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
    let cli_options = CliInterface::from_args();
    init_log(&cli_options.verbose);

    let openai_handler = OpenAIHandler::new_with_token(cli_options.clone().api_auth_token());
    process_cli_request(openai_handler, cli_options).await;

    Ok(())
}

async fn process_cli_request(mut openai_handler: OpenAIHandler, cli_options: CliInterface) {
    match cli_options.args {
        Some(subcommand) => {
            match subcommand {
                CliRequest::CliFiles(request_settings) => {
                    debug!("CliFiles request made");
                    create_file_request(openai_handler, request_settings).await
                },
                CliRequest::CliModels(request_settings) => {
                    debug!("CliModels request made");
                    create_models_request(&mut openai_handler, request_settings).await
                },
                CliRequest::CliFineTune(request_settings) => {
                    debug!("CliFineTune request made");
                    create_finetunes_request(openai_handler, request_settings).await
                },
                CliRequest::CliAudio(request_settings) => {
                    debug!("CliAudio request made");
                    match request_settings.transcriptions() {
                        true => {
                            openai_handler.set_request(OpenAIRequest::OpenAIAudioTranslationRequest(OpenAIAudioTranslationRequest {
                                temperature: request_settings.temperature().to_owned(),
                                response_format: request_settings.response_format().to_owned(),
                                prompt: request_settings.prompt().to_owned(),
                                model: request_settings.model().to_owned(),
                                file: request_settings.file().to_owned(),
                            }));
                            process_response(&mut openai_handler).await
                        }
                        false => {
                            openai_handler.set_request(OpenAIRequest::OpenAIAudioTranscriptionRequest(OpenAIAudioTranscriptionRequest {
                                temperature: request_settings.temperature().to_owned(),
                                response_format: request_settings.response_format().to_owned(),
                                prompt: request_settings.prompt().to_owned(),
                                model: request_settings.model().to_owned(),
                                file: request_settings.file().to_owned(),
                                language: request_settings.language().to_owned(),
                            }));
                            process_response(&mut openai_handler).await
                        }
                    }
                },
            }
        },
        None => {
            create_completions_request(&mut openai_handler, cli_options.to_owned()).await
        },
    }
}

async fn process_response(openai_handler: &mut OpenAIHandler) {
    match openai_handler.process().await {
        Ok(response) => {
            match response {
                OpenAIResponse::OpenAIAudioTranslationResponse(data) => {
                    data.print_response()
                },
                OpenAIResponse::OpenAIAudioTranscriptionResponse(data) => {
                    data.print_response()
                },
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

async fn create_file_upload_request(openai_handler: &mut OpenAIHandler, request_settings: cli::CliFiles, file_path: std::path::PathBuf) {
    openai_handler.set_request(OpenAIRequest::OpenAIFileUploadRequest(OpenAIFileUploadRequest {
        file: file_path.to_owned(),
        purpose: request_settings.purpose().to_owned()
    }));
    process_response(openai_handler).await
}

async fn create_file_request(mut openai_handler: OpenAIHandler, request_settings: cli::CliFiles) {
    match &request_settings.file() {
        Some(file_path) => {
            create_file_upload_request(&mut openai_handler, request_settings, file_path.to_path_buf()).await
        },
        None => {
            create_file_delete_request(&mut openai_handler, request_settings).await
        }
    }
    process_response(&mut openai_handler).await
}

async fn create_finetunes_request(mut openai_handler: OpenAIHandler, request_settings: cli::CliFineTune) {
    match &request_settings.training_file() {
        Some(file) => {
            create_finetune_create_request(&mut openai_handler, request_settings, file.to_owned()).await
        },
        None => {
            match &request_settings.fine_tune_id() {
                Some(fine_tune_id) => {
                    create_finetune_request(&mut openai_handler, request_settings.to_owned(), fine_tune_id.to_owned()).await
                },
                None => {
                    openai_handler.set_request(OpenAIRequest::OpenAIFineTunesRequest(OpenAIFineTunesRequest {}));
                }
            }
        }
    }
    process_response(&mut openai_handler).await
}

async fn create_file_delete_request(openai_handler: &mut OpenAIHandler, request_settings: cli::CliFiles) {
    match request_settings.delete() {
        Some(filename) => {
            openai_handler.set_request(OpenAIRequest::OpenAIFileDeleteRequest(OpenAIFileDeleteRequest { filename }));
        }
        None => {
            openai_handler.set_request(OpenAIRequest::OpenAIFilesRequest(OpenAIFilesRequest {}));
        }
    }
    process_response(openai_handler).await
}

async fn create_finetune_create_request(openai_handler: &mut OpenAIHandler, request_settings: cli::CliFineTune, file: String) {
    openai_handler.set_request(OpenAIRequest::OpenAIFineTuneCreateRequest(OpenAIFineTuneCreateRequest {
        suffix: request_settings.suffix().to_owned(),
        compute_classification_metrics: request_settings.compute_classification_metrics().to_owned(),
        prompt_loss_weight: request_settings.prompt_loss_weight().to_owned(),
        n_epochs: request_settings.n_epochs().to_owned(),
        model: request_settings.model().to_owned(),
        validation_file: request_settings.validation_file().to_owned(),
        training_file: file,
        batch_size: request_settings.batch_size().to_owned(),
        classification_n_classes: request_settings.classification_n_classes().to_owned(),
        classification_positive_class: request_settings.classification_positive_class().to_owned(),
        classification_betas: request_settings.classification_betas().to_owned(),
    }));
    process_response(openai_handler).await
}

async fn create_models_request(openai_handler: &mut OpenAIHandler, request_settings: cli::CliModels) {
    match request_settings.delete {
        Some(model_name) => {
            openai_handler.set_request(OpenAIRequest::OpenAIModelDeleteRequest(OpenAIModelDeleteRequest { model_name }));
        },
        None => {
            openai_handler.set_request(OpenAIRequest::OpenAIModelsRequest(OpenAIModelsRequest {}));
        },
    }
    process_response(openai_handler).await
}

async fn create_finetune_request(openai_handler: &mut OpenAIHandler, request_settings: cli::CliFineTune, fine_tune_id: String) {
    if request_settings.clone().events().to_owned() {
        openai_handler.set_request(OpenAIRequest::OpenAIFineTuneEventsRequest(OpenAIFineTuneEventsRequest {
            fine_tune_id: fine_tune_id.to_owned(),
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
    process_response(openai_handler).await
}

async fn create_completions_request(openai_handler: &mut OpenAIHandler, mut request_settings: cli::CliInterface) {
    openai_handler.set_request(OpenAIRequest::OpenAICompletionsRequest(OpenAICompletionsRequest {
        model: request_settings.model(),
        prompt: request_settings.prompt(),
        max_tokens: request_settings.max_tokens(),
        temperature: request_settings.temperature(),
        user: request_settings.user(),
        logit_bias: request_settings.logit_bias().to_owned(),
        best_of: request_settings.best_of().to_owned(),
        frequency_penalty: request_settings.frequency_penalty().to_owned(),
        presence_penalty: request_settings.presence_penalty().to_owned(),
        stop: request_settings.stop().to_owned(),
        echo: request_settings.echo().to_owned(),
        logprobs: request_settings.logprobs().to_owned(),
        stream: request_settings.stream().to_owned(),
        n: request_settings.n().to_owned(),
        top_p: request_settings.top_p().to_owned(),
        suffix: request_settings.suffix().to_owned(),
    }));
    process_response(openai_handler).await
}
