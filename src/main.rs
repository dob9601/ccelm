use ccelm::Cli;
use ccelm::ConcurrentSolver;
use ccelm::DatasetReader;
use ccelm::Solver;
use ccelm::TrainingExample;
use clap::Parser;
use itertools::Itertools;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let cli = Cli::parse();

    let reader = DatasetReader::new(cli.dataset, cli.metadata.clone())?;

    let training_examples = reader.collect::<Result<Vec<TrainingExample>, Box<dyn Error>>>()?;

    let boundaries = if cli.concurrent {
        let solver = ConcurrentSolver::new(training_examples, &cli.metadata); //, Some(1));
        solver.solve()
    } else {
        let solver = Solver::new(training_examples, &cli.metadata); //, Some(1));
        solver.solve()
    };

    println!(
        "Specific Boundary\n{}",
        boundaries
            .specific_boundary
            .iter()
            .map(|h| { h.to_string() })
            .join("\n")
    );
    println!(
        "General Boundary: {} hypotheses",
        boundaries.general_boundary.len()
    );

    Ok(())
}
