#[macro_use]
extern crate log;

mod openai;
mod cli;
mod cmdln;

use openai::{OpenAIHandler, OpenAICompletionsRequest, OpenAIRequest};
use cli::CliInterface;
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

    let prompt = &cli_options.prompt();
    let token = cli_options.clone().api_auth_token();
    let user = &cli_options.user();
    let model = &cli_options.model();
    let max_tokens = &cli_options.max_tokens();
    let temperature = &cli_options.temperature();

    let mut openai_handler = OpenAIHandler::new_with_token(token);
    openai_handler.set_request(OpenAIRequest::OpenAICompletionsRequest(OpenAICompletionsRequest {
        model: model.to_owned(),
        prompt: prompt.to_owned(),
        max_tokens: max_tokens.to_owned(),
        temperature: temperature.to_owned(),
        user: user.to_owned(),
    }));
    openai_handler.process().await?;
    Ok(())
}

fn init_log(is_verbose: &u8) {
    let environment_override: Option<u8> = match env::var("DEBUG") {
        Ok(value)  => {Some(value.trim().parse().expect("Wanted a number"))},
        Err(error) => {trace!("{}", error);None}
    };
    let is_verbose = match environment_override {Some(value)=>{value},None=>{is_verbose.to_owned()}};
    let logging_level = match is_verbose {
        0 => {LevelFilter::Info},
        1 => {LevelFilter::Warn},
        2 => {LevelFilter::Error},
        3 => {LevelFilter::Debug},
        4 => {LevelFilter::Trace},
        _ => {LevelFilter::max()},
    };
    Builder::new().filter_level(logging_level).init();
}
