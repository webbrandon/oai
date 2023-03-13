mod models;
mod files;

use models::CliModels;
use files::CliFiles;

use structopt::StructOpt;
use structopt::clap::AppSettings::*;
use std::io::{self, BufRead};
use crate::cmdln;

#[derive(Debug, StructOpt, Clone, Default)]
#[structopt(global_settings = &[DisableVersion, DisableHelpSubcommand, DeriveDisplayOrder, VersionlessSubcommands],verbatim_doc_comment)]
/// ChatGBT CLI Buddy
pub struct CliInterface {
	/// Verbose mode (-v, -vv, -vvv, etc.)
	#[structopt(short, long, parse(from_occurrences))]
	pub verbose: u8,
	/// Question
	pub prompt: Option<String>,
	/// ID of the model to use
	#[structopt(long = "model", short = "m", default_value = "text-davinci-003")]
	pub model: String,
	/// The maximum number of tokens
	#[structopt(long = "max-tokens", default_value = "2048")]
	pub max_tokens: usize,
	/// What sampling temperature to use, between 0 and 2
	#[structopt(long = "temperature", short = "t", default_value = "0.5")]
	pub temperature: f32,
	/// API Authorization Token
	#[structopt(long = "api-auth-token", short = "a", env)]
	pub api_auth_token: Option<String>,
	/// User ID (default: session username)
	#[structopt(long = "user", short = "u")]
	pub user: Option<String>,
	#[structopt(subcommand)]
	pub args: Option<CliRequest>,
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(
	global_settings = &[DisableVersion, DisableHelpSubcommand, DeriveDisplayOrder, VersionlessSubcommands],
)]
pub enum CliRequest {
	/// Print list of usable models
	#[structopt(name = "models")]
	CliModels(CliModels),
	/// Print files owned by account
	#[structopt(name = "files")]
	CliFiles(CliFiles),
}

impl CliRequest {
	pub fn process(&self) {
		print!("")
	}
}

impl CliInterface {
	pub fn prompt(&mut self) -> String {
		trace!("prompt value request");
		if self.prompt.is_some() {
			trace!("prompt value provided");
			self.prompt.clone().unwrap()
		} else {
			let stdin = io::stdin();
			let mut input_stream = String::new();
			if atty::isnt(atty::Stream::Stdin) {
				trace!("prompt is coming from stdin");
    			stdin.lock().lines().for_each(|x| {
    				input_stream.push_str(&x.unwrap());
    			});
				input_stream
			} else if atty::is(atty::Stream::Stdin) {
				trace!("no prompt - display help");
				let mut handler = cmdln::CommandLineHandler::new();
				handler.set_exit_on_error(true);

				let output = handler.run_cmd("chatgbt-buddy -h");
				println!("{}",&output);
				std::process::exit(1)
			} else {
				trace!("tell us how you got here");
				std::process::exit(1)
			}
		}
	}

	pub fn model(&mut self) -> String {
		trace!("model value request");
		self.model.clone()
	}

	pub fn user(&mut self) -> String {
		trace!("user value request");
		if self.user.is_some() {
			self.user.clone().unwrap()
		} else {
			trace!("request system username because user not provided");
			whoami::username()
		}
	}

	pub fn max_tokens(&mut self) -> usize {
		trace!("max-tokens value request");
		self.max_tokens.clone()
	}

	pub fn temperature(&mut self) -> f32 {
		trace!("temperature value request");
		self.temperature.clone()
	}

	pub fn api_auth_token(&mut self) -> String {
		trace!("api-auth-token value request");
		match &self.api_auth_token {
		    Some(token) => token.clone(),
		    None => {
				warn!("Please provide API Authorization Token!");
				std::process::exit(0)
			}
		}
	}
}
