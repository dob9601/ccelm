use std::error::Error;
use std::io::Read;

use ccelm::hypothesis::Hypothesis;
use ccelm::Cli;
use clap::Parser;
use log::info;

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let cli = Cli::parse();

    let reader = csv::Reader::from_path(cli.input_dataset)?;

    let training_examples = reader
        .into_records()
        .map(|record| record.map(Hypothesis::try_from))
        .collect::<Result<Result<Vec<_>, _>, _>>()??;

    debug_assert!(training_examples
        .iter()
        .all(|example| example.attributes.len() == training_examples[0].attributes.len()));

    let attribute_length = training_examples[0].attributes.len();

    // FIXME: Most examples treat this as a single value rather than a vec, double check? book
    // implies otherwise
    let mut specific_hypotheses = vec![Hypothesis::specific(attribute_length)];
    let mut general_hypotheses = vec![Hypothesis::general(attribute_length)];

    for example in training_examples.into_iter() {
        println!("{specific_hypotheses:?}");
        println!("{general_hypotheses:?}");
        println!("Training example: {example}");
        std::io::stdin().read_exact(&mut [0u8]).unwrap();

        info!("Processing training example: {example}");
        if example.is_positive {
            // Remove any hypothesis that is inconsistent with d
            general_hypotheses.retain(|hypothesis| hypothesis.is_consistent(&example));

            specific_hypotheses = specific_hypotheses
                .into_iter()
                .map(|hypothesis| hypothesis.generalize(&example).unwrap())
                .collect();
        } else {
            // Remove any hypothesis that is inconsistent with d
            specific_hypotheses.retain(|hypothesis| hypothesis.is_consistent(&example));

            general_hypotheses = general_hypotheses
                .into_iter()
                .flat_map(|hypothesis| hypothesis.specialize(&example).unwrap())
                .collect();
        }
    }
    println!("{specific_hypotheses:?}");
    println!("{general_hypotheses:?}");

    Ok(())
}
