use structopt::StructOpt;
use structopt::clap::AppSettings::*;
use std::path::PathBuf;

#[derive(Debug, StructOpt, Clone, Default)]
#[structopt(global_settings = &[DisableVersion, DisableHelpSubcommand, DeriveDisplayOrder, VersionlessSubcommands])]
pub struct CliImage {
    /// Text description of the desired image
    pub prompt: Option<String>,
    /// Output location for generated content
    #[structopt(long = "save", default_value = "")]
    pub out_path: PathBuf,
    /// Image to edit
    #[structopt(long = "image", short = "i")]
    pub image: Option<PathBuf>,
    /// Image indicating where image should be edited
    #[structopt(long = "mask", short = "m")]
    pub mask: Option<PathBuf>,
    /// Number of images to generate
    #[structopt(long = "n", short = "n", default_value = "1")]
    pub n: u32,
    /// Translate foriegn language to native language
    #[structopt(long = "size", short = "s", default_value = "1024x1024")]
    pub size: String,
    /// Format in which the generated images are returned
    #[structopt(long = "response-format", short = "f", default_value = "b64_json")]
    pub response_format: String,
	/// User ID (default: session username)
	#[structopt(long = "user", short = "u")]
    pub user: Option<String>,
 }

impl CliImage {
    pub fn user(&mut self) -> String {
        trace!("user value request");
        if self.user.is_some() {
            self.user.clone().unwrap()
        } else {
            trace!("request system username because user not provided");
            whoami::username()
        }
    }

    /// Get a reference to the cli image's prompt.
    pub fn is_prompt(&self) -> bool {
        match &self.prompt {
            Some(_) => true,
            None => false,
        }
    }

    /// Get a reference to the cli image's prompt.
    pub fn prompt(&self) -> String {
        match &self.prompt {
            Some(prompt) => prompt.to_owned(),
            None => String::new()
        }
    }

    /// Get a reference to the cli image's image.
    pub fn image(&self) -> &Option<PathBuf> {
        &self.image
    }

    /// Get a reference to the cli image's mask.
    pub fn mask(&self) -> &Option<PathBuf> {
        &self.mask
    }

    /// Get a reference to the cli image's n.
    pub fn n(&self) -> &u32 {
        &self.n
    }

    /// Get a reference to the cli image's size.
    pub fn size(&self) -> &String {
        &self.size
    }

    /// Get a reference to the cli image's response format.
    pub fn response_format(&self) -> &String {
        &self.response_format
    }

    /// Get a reference to the cli image's out path.
    pub fn out_path(&self) -> &PathBuf {
        &self.out_path
    }
}
