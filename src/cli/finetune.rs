use structopt::StructOpt;
use structopt::clap::AppSettings::*;

#[derive(Debug, StructOpt, Clone, Default)]
#[structopt(global_settings = &[DisableVersion, DisableHelpSubcommand, DeriveDisplayOrder, VersionlessSubcommands])]
pub struct CliFineTune {
    /// The ID fine-tuning job
    pub fine_tune_id: Option<String>,
    /// The ID fine-tuning job
	#[structopt(long = "events", short = "e")]
    pub events: bool,
    /// The ID of an uploaded file that contains training data
	#[structopt(long = "training-file", short = "t")]
    pub training_file: Option<String>,
    /// The ID of an uploaded file that contains validation data
	#[structopt(long = "validation-file", short = "v")]
    pub validation_file: Option<String>,
    /// The name of the base model to fine-tune
	#[structopt(long = "model", short = "m", default_value = "davinci")]
    pub model: String,
    /// The number of epochs to train the model for
	#[structopt(long = "n-epochs", short = "n", default_value = "4")]
    pub n_epochs: i32,
    /// The weight to use for loss on the prompt tokens
	#[structopt(long = "prompt-loss-weight", short = "w", default_value = "0.01")]
    pub prompt_loss_weight: f32,
    /// These metrics can be viewed in the results file
	#[structopt(long = "compute-classification-metrics")]
    pub compute_classification_metrics: bool,
    /// A string of up to 40 characters
	#[structopt(long = "suffix", short = "s")]
    pub suffix: Option<String>,
    /// The batch size
	#[structopt(long = "batch-size", short = "b")]
    batch_size: Option<u32>,
    /// The number of classes
	#[structopt(long = "classification-n-classes")]
    classification_n_classes: Option<u32>,
    /// The positive class in binary 
	#[structopt(long = "classification-positive-class")]
    classification_positive_class: Option<String>,
    /// Only used for binary class
	#[structopt(long = "classification_betas")]
    classification_betas: Option<Vec<String>>,
    /// Cancel a fine-tune job
	#[structopt(long = "cancel", short = "c")]
    pub cancel: bool,
 }

impl CliFineTune {
    /// Get a file if passed.
    pub fn training_file(&self) -> Option<String> {
        self.training_file.as_ref().map(|file_path| file_path.to_owned())
    }

    /// Delete a file
    pub fn cancel(self) -> bool {
        self.cancel.to_owned()
    }

    /// Get a reference to the cli fine tune's validation file.
    pub fn validation_file(&self) -> &Option<String> {
        &self.validation_file
    }

    /// Get a reference to the cli fine tune's model.
    pub fn model(&self) -> &String {
        &self.model
    }

    /// Get a reference to the cli fine tune's n epochs.
    pub fn n_epochs(&self) -> &i32 {
        &self.n_epochs
    }

    /// Get a reference to the cli fine tune's prompt loss weight.
    pub fn prompt_loss_weight(&self) -> &f32 {
        &self.prompt_loss_weight
    }

    /// Get a reference to the cli fine tune's compute classification metrics.
    pub fn compute_classification_metrics(&self) -> &bool {
        &self.compute_classification_metrics
    }

    /// Get a reference to the cli fine tune's suffix.
    pub fn suffix(&self) -> &Option<String> {
        &self.suffix
    }

    /// Get a reference to the cli fine tune's fine tune id.
    pub fn fine_tune_id(&self) -> &Option<String> {
        &self.fine_tune_id
    }

    /// Get a reference to the cli fine tune's events.
    pub fn events(&self) -> &bool {
        &self.events
    }

    /// Get a reference to the cli fine tune's batch size.
    pub fn batch_size(&self) -> &Option<u32> {
        &self.batch_size
    }

    /// Get a reference to the cli fine tune's classification n classes.
    pub fn classification_n_classes(&self) -> &Option<u32> {
        &self.classification_n_classes
    }

    /// Get a reference to the cli fine tune's classification positive class.
    pub fn classification_positive_class(&self) -> &Option<String> {
        &self.classification_positive_class
    }

    /// Get a reference to the cli fine tune's classification betas.
    pub fn classification_betas(&self) -> &Option<Vec<String>> {
        &self.classification_betas
    }
}
