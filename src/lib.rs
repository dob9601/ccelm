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
