use structopt::StructOpt;
use structopt::clap::AppSettings::*;

#[derive(Debug, StructOpt, Clone, Default)]
#[structopt(global_settings = &[DisableVersion, DisableHelpSubcommand, DeriveDisplayOrder, VersionlessSubcommands])]
pub struct CliModels {
 }
