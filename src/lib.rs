mod hypothesis;
pub use hypothesis::Hypothesis;

mod attribute;
pub use attribute::Attribute;

mod training_example;
pub use training_example::TrainingExample;

mod cli;
pub use cli::Cli;

mod reader;
pub use reader::DatasetReader;

pub mod hypothesis_space;

mod solver;
pub use solver::Solver;

mod concurrent;
pub use concurrent::ConcurrentSolver;

#[derive(Default)]
pub struct ComputedBoundaries<'a> {
    pub specific_boundary: Option<Hypothesis<'a>>,
    pub general_boundary: Vec<Hypothesis<'a>>,
}

impl<'a> ComputedBoundaries<'a> {
    pub fn merge(&mut self, mut other: ComputedBoundaries<'a>) {
        match (&self.specific_boundary, other.specific_boundary) {
            (Some(boundary), Some(other_boundary)) => {
                self.specific_boundary = Some(boundary.generalize(&other_boundary.into()))
            }
            (None, Some(other_boundary)) => self.specific_boundary = Some(other_boundary),
            _ => {}
        }


        // self.general_boundary = self.general_boundary.into_iter().flat_map(|hypothesis| {
        //     let specialisations = other.general_boundary.into_iter().
        // })

        self.general_boundary.extend(other.general_boundary);
        // let mut merged_general_boundary = vec![];
        // for hypothesis in other.general_boundary.into_iter() {
        //     if self
        //         .general_boundary
        //         .iter()
        //         .all(|other_hypothesis| !hypothesis.is_more_general(other_hypothesis))
        //     {
        //         let column_data = &hypothesis.dataset_metadata.columns.as_slice();
        //         let specializations = other.general_boundary.iter().flat_map(|other_hypothesis| {
        //             hypothesis.specialize(&(other_hypothesis.clone()).into(), column_data)
        //         }).collect_vec();
        //         merged_general_boundary.push(hypothesis);
        //         merged_general_boundary.extend(specializations);
        //     }
        // }
        // self.general_boundary = merged_general_boundary;
        //
        // // Duplicate the above logic instead??
        // self.general_boundary.retain(|hypothesis| {
        //     other
        //         .general_boundary
        //         .iter()
        //         .all(|other_hypothesis| !hypothesis.is_more_general(other_hypothesis))
        // });
        // other.general_boundary.retain(|hypothesis| {
        //     self.general_boundary
        //         .iter()
        //         .all(|other_hypothesis| !hypothesis.is_more_general(other_hypothesis))
        // });

        // IDEA: COULD DO CONSISTENCY CHECK AT THE END?? i.e. check for consistency after
        // individual solvers have run

        // let mut new_boundary = vec![];
        // for hypothesis in self.general_boundary.iter() {
        //     if self
        //         .general_boundary
        //         .iter()
        //         .all(|h| hypothesis.is_more_general(h) || !hypothesis.is_more_specific(h))
        //     {
        //         new_boundary.push(hypothesis.clone())
        //     } else {
        //         // dbg!("Ditching h");
        //     }
        // }
        //
        // dbg!(&new_boundary);
        // self.general_boundary = new_boundary;

        // self.general_boundary
        //     .retain(|h| self.general_boundary.iter().all(|j| h.is_more_general(j)))

        // self.general_boundary.retain(|hypothesis|)
        // Iterate over new boundary
        // Remove any hypotheses that are more general than old boundary
        //
        // Iterate over old boundary
        // Remove any hypotheses that are more general than new boundary
        //
        // Merge boundaries
    }
}
