use structopt::StructOpt;
use structopt::clap::AppSettings::*;

#[derive(Debug, StructOpt, Clone, Default)]
#[structopt(global_settings = &[DisableVersion, DisableHelpSubcommand, DeriveDisplayOrder, VersionlessSubcommands])]
pub struct CliEmbeddings {
    /// Input text to get embeddings
    pub input: String,

    /// ID of the model to use
    #[structopt(long = "model", short = "m", default_value = "text-embedding-ada-002")]
    pub model: String,

	/// User ID (default: session username)
	#[structopt(long = "user", short = "u")]
	pub user: Option<String>,
 }

impl CliEmbeddings {
    /// Get a reference to the cli embeddings's user.
	pub fn user(&mut self) -> String {
		trace!("user value request");
		if self.user.is_some() {
			self.user.clone().unwrap()
		} else {
			trace!("request system username because user not provided");
			whoami::username()
		}
	}

    /// Get a reference to the cli embeddings's model.
    pub fn model(&self) -> &String {
        &self.model
    }

    /// Get a reference to the cli embeddings's input.
    pub fn input(&self) -> &String {
        &self.input
    }
}
