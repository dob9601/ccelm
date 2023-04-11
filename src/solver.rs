use log::{info, trace};

use crate::reader::DatasetMetadata;
use crate::{ComputedBoundaries, Hypothesis, TrainingExample};

#[derive(Clone, Debug)]
pub struct Solver<'a> {
    specific_boundary: Hypothesis<'a>,
    general_boundary: Vec<Hypothesis<'a>>,
    training_examples: Vec<TrainingExample>,
    dataset_metadata: &'a DatasetMetadata,
}

impl<'a> Solver<'a> {
    pub fn new(
        training_examples: Vec<TrainingExample>,
        dataset_metadata: &'a DatasetMetadata,
    ) -> Self {
        let attribute_count = dataset_metadata.columns.len();
        Self {
            specific_boundary: Hypothesis::specific(attribute_count, dataset_metadata),
            general_boundary: vec![Hypothesis::general(attribute_count, dataset_metadata)],
            training_examples,
            dataset_metadata,
        }
    }

    pub fn solve(mut self) -> ComputedBoundaries<'a> {
        let n_training_examples = self.training_examples.len();
        let column_data = &self.dataset_metadata.columns;
        for (index, example) in self.training_examples.into_iter().enumerate() {
            info!(
                "{index}/{n_training_examples} | {} general hypotheses",
                self.general_boundary.len()
            );

            if example.is_positive {
                info!("Processing positive training example: {example}");
                // Remove any hypothesis that is inconsistent with d
                self.general_boundary
                    .retain(|hypothesis| hypothesis.is_consistent(&example));

                trace!("Inconsistent hypotheses removed from general boundary");

                self.specific_boundary = self.specific_boundary.generalize(&example).unwrap();

                trace!("Specific hypothesis barrier refined");
            } else {
                info!("Processing negative training example: {example}");
                // Remove any hypothesis that is inconsistent with d
                //specific_hypothesis.retain(|hypothesis| hypothesis.is_consistent(&example));

                let mut new_hypotheses = vec![];
                for hypothesis in self.general_boundary.into_iter() {
                    if hypothesis.is_consistent(&example) {
                        new_hypotheses.push(hypothesis);
                        continue;
                    }

                    let mut specializations =
                        hypothesis.specialize(&example, column_data.as_slice());
                    specializations.retain(|specialization| {
                        specialization.is_more_general(&self.specific_boundary)
                    });
                    new_hypotheses.extend(specializations);
                }

                self.general_boundary = new_hypotheses;
            }

            info!("Successfully processed example");
        }

        ComputedBoundaries {
            specific_boundary: self.specific_boundary,
            general_boundary: self.general_boundary,
        }
    }
}
