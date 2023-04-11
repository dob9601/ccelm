use std::sync::{Arc, Mutex};
use std::thread;

use itertools::Itertools;

use crate::reader::DatasetMetadata;
use crate::solver::Solver;
use crate::{ComputedBoundaries, TrainingExample};

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
        let boundaries = Arc::new(Mutex::new(ComputedBoundaries {
            specific_boundary: vec![],
            general_boundary: vec![],
        }));

        crossbeam::scope(|scope| {
            self.solvers.into_iter().for_each(|solver| {
                scope.spawn(|_| {
                    let new_boundaries = solver.solve();

                    boundaries.lock().unwrap().merge(new_boundaries);
                });
            });
        })?;

        let extracted_boundaries = std::mem::take(&mut *boundaries.lock().unwrap());

        Ok(extracted_boundaries)
    }
}
