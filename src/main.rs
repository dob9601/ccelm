use ccelm::Cli;
use ccelm::ConcurrentSolver;
use ccelm::DatasetReader;
use ccelm::Solver;
use ccelm::TrainingExample;
use clap::Parser;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let cli = Cli::parse();

    let delimiter = if cli.tabs { b'\t' } else { b',' };

    let reader =
        DatasetReader::new(cli.input_dataset, cli.dataset_metadata.clone(), delimiter)?;

    let training_examples = reader.collect::<Result<Vec<TrainingExample>, Box<dyn Error>>>()?;

    let solver = ConcurrentSolver::new(training_examples, &cli.dataset_metadata);
    let boundaries = solver.solve().unwrap();

    println!("Specific Boundary {:?}", boundaries.specific_boundary);
    println!(
        "General Boundary: {} hypotheses",
        boundaries.general_boundary.len()
    );

    Ok(())
}
