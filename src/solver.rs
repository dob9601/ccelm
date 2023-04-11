use crate::reader::DatasetMetadata;
use crate::{Hypothesis, TrainingExample, ComputedBoundaries};

#[derive(Clone, Debug)]
pub struct Solver<'a> {
    specific_boundary: Hypothesis<'a>,
    general_boundary: Vec<Hypothesis<'a>>,
    training_examples: Vec<TrainingExample>,
    dataset_metadata: &'a DatasetMetadata
}

impl<'a> Solver<'a> {
    pub fn new(training_examples: Vec<TrainingExample>, dataset_metadata: &'a DatasetMetadata) -> Self {
        let attribute_count = dataset_metadata.columns.len();
        Self {
            specific_boundary: Hypothesis::specific(attribute_count, dataset_metadata),
            general_boundary: vec![Hypothesis::general(attribute_count, dataset_metadata)],
            training_examples,
            dataset_metadata
        }
    }

    pub fn solve(self) -> ComputedBoundaries<'a> {
        todo!()
    }
}
