use structopt::StructOpt;
use structopt::clap::AppSettings::*;

#[derive(Debug, StructOpt, Clone, Default)]
#[structopt(global_settings = &[DisableVersion, DisableHelpSubcommand, DeriveDisplayOrder, VersionlessSubcommands])]
pub struct CliModels {
    /// Delete a model from account (eg: curie:ft-acmeco-2021-03-03-21-44-20)
    #[structopt(long = "delete", short = "d")]
    pub delete: Option<String>,
 }

impl CliModels {
    /// Get a reference to the cli models's delete.
    pub fn delete(&self) -> &Option<String> {
        &self.delete
    }
}
