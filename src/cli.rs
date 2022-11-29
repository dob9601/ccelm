use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, help = "Path to the input dataset of training examples")]
    pub input_dataset: PathBuf,

    #[arg(short, long, help = "Path to output all valid hypotheses to")]
    pub output_path: Option<PathBuf>,
}
