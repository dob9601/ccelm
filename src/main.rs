use std::collections::HashSet;
use std::error::Error;

use ccelm::Attribute;
use ccelm::Cli;
use ccelm::DatasetReader;
use ccelm::Hypothesis;
use ccelm::TrainingExample;
use clap::Parser;
use log::info;

use std::alloc;
use cap::Cap;

// #[global_allocator]
// static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, 8 * 1024 * 1024 * 1024);

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let cli = Cli::parse();

    let column_data = cli.dataset_metadata.columns.clone();

    let delimiter = if cli.tabs {
        b'\t'
    } else {
        b','
    };

    let mut reader = DatasetReader::new(cli.input_dataset, cli.dataset_metadata, delimiter)?;
    let attribute_length = reader.attributes()?.len();

    let mut specific_hypothesis = Hypothesis::specific(attribute_length);
    // PERF: Try running with hashset and see if it helps
    let mut general_hypotheses = vec![Hypothesis::general(attribute_length)];
    
    let training_examples = reader.into_iter().collect::<Result<Vec<TrainingExample>, Box<dyn Error>>>()?;

    let training_example_count = training_examples.len();

    for (index, example) in training_examples.into_iter().enumerate() {
        // Reading a row from a CSV is fallible. Unwrap the inner value first.

        println!("{index}/{training_example_count} | {} general hypotheses", general_hypotheses.len());

        // println!(
        //     "General Boundary {:#?}",
        //     general_hypotheses
        //         .iter()
        //         .map(|h| h.to_string())
        //         .collect::<Vec<String>>()
        // );

        // println!("Specific Boundary {specific_hypothesis}");
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

            general_hypotheses.sort();
            general_hypotheses.dedup();
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
