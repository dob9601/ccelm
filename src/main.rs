use std::error::Error;
use std::io::Read;

use ccelm::Hypothesis;
use ccelm::Cli;
use ccelm::TrainingExample;
use clap::Parser;
use log::info;

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let cli = Cli::parse();

    let column_data = cli.dataset_metadata.columns;

    let reader = csv::Reader::from_path(cli.input_dataset)?;

    let training_examples = reader
        .into_records()
        .map(|record| record.map(TrainingExample::try_from))
        .collect::<Result<Result<Vec<_>, _>, _>>()??;

    debug_assert!(training_examples
        .iter()
        .all(|example| example.attributes.len() == training_examples[0].attributes.len()));
    debug_assert!(training_examples[0].attributes.len() == column_data.len());

    let attribute_length = training_examples[0].attributes.len();

    // FIXME: Most examples treat this as a single value rather than a vec, double check? book
    // implies otherwise
    let mut specific_hypothesis = Hypothesis::specific(attribute_length);
    let mut general_hypotheses = vec![Hypothesis::general(attribute_length)];

    for example in training_examples.into_iter() {
        println!("{specific_hypothesis}");
        println!("{general_hypotheses:?}");
        println!("Training example: {example}");
        std::io::stdin().read_exact(&mut [0u8]).unwrap();

        info!("Processing training example: {example}");
        if example.is_positive {
            // Remove any hypothesis that is inconsistent with d
            general_hypotheses.retain(|hypothesis| hypothesis.is_consistent(&example));

            specific_hypothesis = specific_hypothesis.generalize(&example).unwrap();
        } else {
            // Remove any hypothesis that is inconsistent with d
            //specific_hypothesis.retain(|hypothesis| hypothesis.is_consistent(&example));

            general_hypotheses = general_hypotheses
                .into_iter()
                // TODO: Technically should append original hypothesis here and not just map it
                .flat_map(|hypothesis| hypothesis.specialize(&example, column_data.as_slice()).unwrap())
                .filter(|general_hypothesis| {
                    match general_hypothesis.partial_cmp(&specific_hypothesis) {
                        Some(comparison) => !comparison.is_lt(),
                        None => true,
                    }
                })
                .collect();
        }
    }
    println!("{specific_hypothesis:?}");
    println!("{general_hypotheses:?}");

    Ok(())
}
