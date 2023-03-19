mod models;
mod files;
mod finetune;
mod audio;
mod images;

pub use models::CliModels;
pub use files::CliFiles;
pub use finetune::CliFineTune;
pub use audio::CliAudio;
pub use images::CliImage;
use std::fs;
use std::net::SocketAddr;
use std::error::Error;
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
	/// Instructions how to edit the prompt
	pub instruction: Option<String>,
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
	/// After a completion of inserted text
	#[structopt(long = "suffix", short = "s")]
    pub suffix: Option<String>,
	/// Aalternative to sampling with temperature
	#[structopt(long = "top-p", default_value = "1")]
    pub top_p: f32,
	/// How many completions to generate
	#[structopt(long = "n", short = "n", default_value = "1")]
    pub n: u32,
	/// Stream back partial progress
	#[structopt(long = "stream")]
    pub stream: bool,
	/// Probabilities most likely tokens, as well the chosen tokens
	#[structopt(long = "logprobs", short = "l")]
    pub logprobs: Option<u32>,
	/// Echo back the prompt in addition
	#[structopt(long = "echo", short = "e")]
    pub echo: bool,
	/// Returned text will not contain the stop sequence
	#[structopt(long = "stop")]
    pub stop: Option<Vec<String>>,
	/// Penalize new tokens based on whether they appear in the text so far
	#[structopt(long = "presence-penalty", short = "p", default_value = "0")]
    pub presence_penalty: f32,
	/// Penalize new tokens based on their existing frequency in the text so far
	#[structopt(long = "frequency-penalty", short = "f", default_value = "0")]
    pub frequency_penalty: f32,
	/// Highest log probability per token
	#[structopt(long = "best-of", short = "b", default_value = "1")]
    pub best_of: u32,
	/// Likelihood of specified tokens appearing
	#[structopt(long = "logit-bias")]
    pub logit_bias: Option<String>,

	#[structopt(subcommand)]
	pub args: Option<CliRequest>,
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(
	global_settings = &[DisableVersion, DisableHelpSubcommand, DeriveDisplayOrder, VersionlessSubcommands],
)]
pub enum CliRequest {
	/// List of usable models
	#[structopt(name = "models")]
	CliModels(CliModels),
	/// List, upload or remove files for account
	#[structopt(name = "files")]
	CliFiles(CliFiles),
	/// List, create, or cancel fine-tune jobs
	#[structopt(name = "fine-tunes")]
	CliFineTune(CliFineTune),
	/// Transcribe or translate audio to text
	#[structopt(name = "audio")]
	CliAudio(CliAudio),
	/// Generate new, edited or variation images
	#[structopt(name = "image")]
	CliImage(CliImage),
}

impl CliRequest {
	pub fn process(&self) {
		print!("")
	}
}

impl CliInterface {
	pub async fn prompt(mut self) -> String {
		trace!("prompt value request");
		if self.clone().prompt.is_some() {
			trace!("prompt value provided");
			match self.clone().prompt_string().await {
			    Some(prompt_response) => {
					prompt_response.to_owned()
				}
			    None => {
					self.clone().prompt.unwrap()
				}
			}
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
	async fn prompt_string(mut self) -> Option<String> {
		match &self.prompt {
			Some(prompt) => {
				let first_char = prompt.chars().nth(0).clone().unwrap();
				if self.clone().is_file_flag(first_char) {
					debug!("attempting to open file for context: {}", prompt.clone().replace("@", ""));
					match fs::read_to_string(prompt.clone().replace("@", "")) {
						Ok(file_content) => {
							self.prompt = Some(file_content);
							self.prompt
						}
						Err(err) => {
							warn!("There was an error opening file: {:#?}", err);
							self.prompt
						}
					}
				} else {
					self.prompt
				}
			}
			None => {self.prompt},
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

    /// Get a reference to the cli interface's suffix.
    pub fn suffix(&self) -> &Option<String> {
        &self.suffix
    }

    /// Get a reference to the cli interface's top p.
    pub fn top_p(&self) -> &f32 {
        &self.top_p
    }

    /// Get a reference to the cli interface's n.
    pub fn n(&self) -> &u32 {
        &self.n
    }

    /// Get a reference to the cli interface's stream.
    pub fn stream(&self) -> &bool {
        &self.stream
    }

    /// Get a reference to the cli interface's logprobs.
    pub fn logprobs(&self) -> &Option<u32> {
        &self.logprobs
    }

    /// Get a reference to the cli interface's echo.
    pub fn echo(&self) -> &bool {
        &self.echo
    }

    /// Get a reference to the cli interface's stop.
    pub fn stop(&self) -> &Option<Vec<String>> {
        &self.stop
    }

    /// Get a reference to the cli interface's presence penalty.
    pub fn presence_penalty(&self) -> &f32 {
        &self.presence_penalty
    }

    /// Get a reference to the cli interface's frequency penalty.
    pub fn frequency_penalty(&self) -> &f32 {
        &self.frequency_penalty
    }

    /// Get a reference to the cli interface's best of.
    pub fn best_of(&self) -> &u32 {
        &self.best_of
    }

    /// Get a reference to the cli interface's logit bias.
    pub fn logit_bias(&self) -> &Option<String> {
        &self.logit_bias
    }

    /// Get a reference to the cli interface's instruction.
    pub async fn instruction(mut self) -> Option<String> {
		match &self.instruction {
		    Some(instruction) => {
				let first_char = instruction.chars().nth(0).clone().unwrap();
				if self.clone().is_file_flag(first_char) {
					debug!("attempting to open file for context: {}", instruction.replace("@", ""));
					match fs::read_to_string(instruction.replace("@", "")) {
					    Ok(file_content) => {
							self.instruction = Some(file_content);
							self.instruction
						}
					    Err(err) => {
							warn!("There was an error opening file: {:#?}", err);
							self.instruction
						}
					}
				} else {
					self.instruction
				}
			}
		    None => {self.instruction},
		}
    }

	pub fn is_file_flag(mut self, segment: char) -> bool {
		if segment == '@' {
			true
		} else {
			false
		}
	}
}
