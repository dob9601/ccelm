use std::thread;

use itertools::Itertools;

use crate::reader::DatasetMetadata;
use crate::solver::Solver;
use crate::{TrainingExample, ComputedBoundaries};

pub struct ConcurrentSolver<'a> {
    solvers: Vec<Solver<'a>>,
}

impl<'a> ConcurrentSolver<'a> {
    pub fn new(
        training_examples: Vec<TrainingExample>,
        dataset_metadata: &'a DatasetMetadata,
    ) -> Self {
        let n_solvers = std::thread::available_parallelism().unwrap();

        let mut chunks: Vec<Vec<TrainingExample>> = vec![vec![]; n_solvers.into()];

        for (index, training_example) in training_examples.into_iter().enumerate() {
            let chunk_index = index % n_solvers;
            chunks[chunk_index].push(training_example)
        }

        let solvers = chunks
            .into_iter()
            .map(|chunk| Solver::new(chunk, dataset_metadata))
            .collect_vec();

        Self { solvers }
    }

    pub fn solve(self) -> thread::Result<ComputedBoundaries<'a>> {
        let mut specific_boundaries = vec![];
        let mut general_boundaries = vec![];

        crossbeam::scope(|scope| {
            self.solvers.into_iter().for_each(|solver| {
                scope.spawn(move |_| {
                    solver.solve();
                });
            });
        })?;
        Ok(())
    }
}
