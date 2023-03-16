use structopt::StructOpt;
use structopt::clap::AppSettings::*;
use std::path::PathBuf;

#[derive(Debug, StructOpt, Clone, Default)]
#[structopt(global_settings = &[DisableVersion, DisableHelpSubcommand, DeriveDisplayOrder, VersionlessSubcommands])]
pub struct CliAudio {
    /// Audio file that will be sent for processing
    /// (formats: mp3, mp4, mpeg, mpga, m4a, wav, or webm)
    pub file: PathBuf,
    /// Turn speech to text
    #[structopt(long = "transcriptions")]
    pub transcriptions: bool,
    /// Translate foriegn language to native language
    #[structopt(long = "translations")]
    pub translations: bool,
    /// ID of the model
    #[structopt(long = "model", short = "m", default_value = "whisper-1")]
    pub model: String,
    /// An optional text to guide the model
    #[structopt(long = "prompt", short = "p")]
    pub prompt: Option<String>,
    /// Format of the transcript output
    /// Options: json, text, srt, verbose_json, or vtt.
    #[structopt(long = "response-format", short = "r", default_value = "json")]
    pub response_format: String,
    /// Sampling temperature, between 0 and 1
    #[structopt(long = "temperature", short = "t", default_value = "0")]
    pub temperature: f32,
    /// Language of the input audio
    #[structopt(long = "language", short = "l")]
    pub language: Option<String>,
 }

impl CliAudio {
    /// Get a reference to the cli audio's transcriptions.
    pub fn transcriptions(&self) -> bool {
        match self.translations() {
            true => false,
            false => true,
        }
    }

    /// Get a reference to the cli audio's translations.
    pub fn translations(&self) -> &bool {
        &self.translations
    }

    /// Get a reference to the cli audio's language.
    pub fn language(&self) -> &Option<String> {
        &self.language
    }

    /// Get a reference to the cli audio's temperature.
    pub fn temperature(&self) -> &f32 {
        &self.temperature
    }

    /// Get a reference to the cli audio's response format.
    pub fn response_format(&self) -> &String {
        &self.response_format
    }

    /// Get a reference to the cli audio's prompt.
    pub fn prompt(&self) -> &Option<String> {
        &self.prompt
    }

    /// Get a reference to the cli audio's model.
    pub fn model(&self) -> &String {
        &self.model
    }

    /// Get a reference to the cli audio's file.
    pub fn file(&self) -> &PathBuf {
        &self.file
    }
}
