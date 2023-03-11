use ccelm::Cli;
use ccelm::DatasetReader;
use ccelm::Hypothesis;
use ccelm::TrainingExample;
use clap::Parser;
use itertools::Itertools;
use log::info;
use log::trace;
use std::error::Error;

// use cap::Cap;
// use std::alloc;
// #[global_allocator]
// static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, 8 * 1024 * 1024 * 1024);

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let cli = Cli::parse();

    let column_data = cli.dataset_metadata.columns.clone();

    let delimiter = if cli.tabs { b'\t' } else { b',' };

    let mut reader =
        DatasetReader::new(cli.input_dataset, cli.dataset_metadata.clone(), delimiter)?;
    let attribute_length = reader.attributes()?.len();

    let mut specific_hypothesis = Hypothesis::specific(attribute_length, &cli.dataset_metadata);

    let mut general_hypotheses = vec![Hypothesis::general(attribute_length, &cli.dataset_metadata)];

    let training_examples = reader
        .collect::<Result<Vec<TrainingExample>, Box<dyn Error>>>()?;

    let training_example_count = training_examples.len();

    for (index, example) in training_examples.into_iter().enumerate() {
        info!(
            "{index}/{training_example_count} | {} general hypotheses",
            general_hypotheses.len()
        );

        if example.is_positive {
            info!("Processing positive training example: {example}");
            // Remove any hypothesis that is inconsistent with d
            general_hypotheses.retain(|hypothesis| hypothesis.is_consistent(&example));

            trace!("Inconsistent hypotheses removed from general boundary");

            specific_hypothesis = specific_hypothesis.generalize(&example).unwrap();

            trace!("Specific hypothesis barrier refined");
        } else {
            info!("Processing negative training example: {example}");
            // Remove any hypothesis that is inconsistent with d
            //specific_hypothesis.retain(|hypothesis| hypothesis.is_consistent(&example));

            let mut new_hypotheses = vec![];
            for hypothesis in general_hypotheses.into_iter() {
                if hypothesis.is_consistent(&example) {
                    new_hypotheses.push(hypothesis);
                    continue
                }

                let mut specializations = hypothesis.specialize(&example, column_data.as_slice());
                specializations.retain(|specialization| specialization.is_more_general(&specific_hypothesis));
                new_hypotheses.extend(specializations);
            }

            general_hypotheses = new_hypotheses;
        }

        info!("Successfully processed example");
        println!("Specific Boundary {specific_hypothesis}");
    }
    println!("Specific Boundary {specific_hypothesis}");
    println!(
        "General Boundary: {}",
        general_hypotheses
            .iter()
            .map(|h| h.to_string())
            .join("\n")
    );

    Ok(())
}
