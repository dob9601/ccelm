use std::error::Error;
use std::io::Read;

use ccelm::Cli;
use ccelm::Hypothesis;
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

    // PERF: Most examples treat this as a single value rather than a vec, double check? book
    // implies otherwise
    let mut specific_hypothesis = Hypothesis::specific(attribute_length);
    let mut general_hypotheses = vec![Hypothesis::general(attribute_length)];

    for example in training_examples.into_iter() {
        println!("Specific Boundary {specific_hypothesis}");
        println!(
            "General Boundary {:?}",
            general_hypotheses
                .iter()
                .map(|h| h.to_string())
                .collect::<Vec<String>>()
        );
        println!("Training example: {example}\n");

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
                // PERF: Technically should append original hypothesis here and not just map it. At
                // the same time, specializing a hypothesis should render the other one more
                // general
                .flat_map(|hypothesis| {
                    hypothesis
                        .specialize(&example, column_data.as_slice())
                        .unwrap()
                })
                .filter(|general_hypothesis| {
                    general_hypothesis.is_more_general(&specific_hypothesis)
                })
                .collect();
        }
    }
    println!("Specific Boundary {specific_hypothesis}");
    println!(
        "General Boundary {:?}",
        general_hypotheses
            .iter()
            .map(|h| h.to_string())
            .collect::<Vec<String>>()
    );

    Ok(())
}
