use std::error::Error;
use std::path::PathBuf;

use clap::Parser;
use serde::Deserialize;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, help = "Path to the input dataset of training examples")]
    pub input_dataset: PathBuf,

    #[arg(
        short,
        long,
        help = "Path to the dataset metadata file. Contains data surrounding ",
        value_parser = parse_dataset_metadata
    )]
    pub dataset_metadata: DatasetMetadata,

    #[arg(short, long, help = "Path to output all valid hypotheses to")]
    pub output_path: Option<PathBuf>,
}

fn parse_dataset_metadata(path: &str) -> Result<DatasetMetadata, Box<dyn Error + Sync + Send>> {
    let file = std::fs::File::open(path)?;
    Ok(serde_yaml::from_reader(file)?)
}

#[derive(Deserialize, Clone)]
pub struct DatasetMetadata {
    pub columns: Vec<Vec<String>>,
}
