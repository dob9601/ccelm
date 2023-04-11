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

mod concurrent;

struct ComputedBoundaries<'a> {
    specific_boundary: Hypothesis<'a>,
    general_boundary: Vec<Hypothesis<'a>>
}
