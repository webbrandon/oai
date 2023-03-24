use structopt::StructOpt;
use structopt::clap::AppSettings::*;
use std::path::PathBuf;

#[derive(Debug, StructOpt, Clone, Default)]
#[structopt(global_settings = &[DisableVersion, DisableHelpSubcommand, DeriveDisplayOrder, VersionlessSubcommands])]
pub struct CliFiles {
    /// File path to upload
    pub file: Option<PathBuf>,
    /// Purpose for upload
	#[structopt(long = "purpose", short = "p", default_value = "fine-tune")]
    pub purpose: String,
    /// Delete a file (eg: file-XjGxS3KTG0uNmNOK362iJua3)
	#[structopt(long = "delete", short = "d")]
    pub delete: Option<String>,
 }

impl CliFiles {
    /// Get a file if passed.
    pub fn file(&self) -> Option<PathBuf> {
        self.file.as_ref().map(|file_path| file_path.to_owned())
    }

    /// Get a reference to the cli files's purpose.
    pub fn purpose(&self) -> &String {
        &self.purpose
    }

    /// Delete a file
    pub fn delete(&self) -> Option<String> {
        self.delete.to_owned()
    }
}
