use std::error::Error;

use ccelm::Cli;
use ccelm::DatasetReader;
use ccelm::Hypothesis;
use clap::Parser;
use log::info;

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let cli = Cli::parse();

    let column_data = cli.dataset_metadata.columns.clone();

    let mut reader = DatasetReader::new(cli.input_dataset, cli.dataset_metadata)?;
    let attribute_length = reader.attributes()?.len();

    let mut specific_hypothesis = Hypothesis::specific(attribute_length);
    let mut general_hypotheses = vec![Hypothesis::general(attribute_length)];

    for maybe_example in reader {
        // Reading a row from a CSV is fallible. Unwrap the inner value first.
        let example = maybe_example?;

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
