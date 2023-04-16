use std::error::Error;
use std::path::PathBuf;

use clap::Parser;

use crate::reader::DatasetMetadata;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, help = "Path to the input dataset of training examples")]
    pub input_dataset: PathBuf,

    #[arg(
        short,
        long,
        help = "Path to a YAML-formatted dataset metadata file. Contains metadata describing the dataset itself.",
        value_parser = parse_dataset_metadata
    )]
    pub dataset_metadata: DatasetMetadata,

    #[arg(short, long, help = "Path to output all valid hypotheses to")]
    pub output_path: Option<PathBuf>,

    #[arg(long, help = "Whether to use the concurrent solver", required = false)]
    pub concurrent: bool
}

fn parse_dataset_metadata(path: &str) -> Result<DatasetMetadata, Box<dyn Error + Sync + Send>> {
    let file = std::fs::File::open(path)?;
    Ok(serde_yaml::from_reader(file)?)
}
