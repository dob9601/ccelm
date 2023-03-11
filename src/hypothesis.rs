use std::fmt::Display;
use std::str::ParseBoolError;

use derivative::Derivative;

use crate::attribute::Attribute;
use crate::reader::DatasetMetadata;
use crate::training_example::TrainingExample;

#[derive(Clone, Derivative)]
#[derivative(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Hypothesis<'a> {
    pub attributes: Vec<Attribute>,

    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    dataset_metadata: &'a DatasetMetadata,
}

// FIXME: Should hypothesis be renamed to training example? Actual hypotheses all seem to be true
impl<'a> Hypothesis<'a> {
    // pub fn enumerate_hypotheses(
    //     _specific_boundary: Hypothesis,
    //     _general_hypotheses: Vec<Hypothesis>,
    // ) -> Vec<Hypothesis> {
    //     // let mut hypotheses = vec![];
    //     //
    //     // let zipped_attributes = specific_boundary.attributes.iter().zip(general_hypotheses.attributes.iter())
    //     //
    //     // for attribute in specific_boundary.attributes.iter() {
    //     //     // FIXME: RESUME HERE <_____------------
    //     // }
    //
    //     todo!()
    // }

    pub fn general(n_attributes: usize, dataset_metadata: &'a DatasetMetadata) -> Self {
        Self {
            attributes: vec![Attribute::Any; n_attributes],
            dataset_metadata,
        }
    }

    pub fn specific(n_attributes: usize, dataset_metadata: &'a DatasetMetadata) -> Self {
        Self {
            attributes: vec![Attribute::NoValue; n_attributes],
            dataset_metadata,
        }
    }

    pub fn classify(&self, training_example: &TrainingExample) -> bool {
        debug_assert_eq!(self.attributes.len(), training_example.attributes.len());

        // TODO: Take into account training example is_positive

        self.attributes
            .iter()
            .zip(training_example.attributes.iter())
            .all(|(attribute, other_attribute)| {
                if let (Attribute::Value(left), Attribute::Value(right)) =
                    (attribute, other_attribute)
                {
                    left == right
                } else {
                    other_attribute <= attribute
                }
            })
    }

    /// Implement a partial ordering for hypotheses in terms of generality - i.e. the more
    /// general hypothesis is ranked higher.
    ///
    /// Machine Learning by Tom Mitchell defines the generality/specificity ordering of hypotheses
    /// as:
    /// $$
    /// (\forall x \in X)[(h_k(x)=1)\implies (h_j(x)=1)]
    /// $$
    /// For $h_k$ to be more specific than $h_j$, if $h_j$ satisfies a hypothesis, $h_k$ must also do so
    ///
    /// Therefore, if every attribute in hypothesis $h_k$ is more specific than that of $h_j$, it
    /// itself must be more specific
    pub fn is_more_general(&self, other: &Self) -> bool {
        self.attributes
            .iter()
            .zip(other.attributes.iter())
            .all(|(attribute, other_attribute)| {
                if let (Attribute::Value(left), Attribute::Value(right)) =
                    (attribute, other_attribute)
                {
                    // If both are values, can only be more general (or equal) if attributes are
                    // the same
                    left == right
                } else {
                    attribute >= other_attribute
                }
            })
    }

    // FIXME: Replace this with a classify function. Hypothesis is consistent if it correctly
    // classifies the other hypothesis
    pub fn is_consistent(&self, training_example: &TrainingExample) -> bool {
        debug_assert_eq!(self.attributes.len(), training_example.attributes.len());

        let classification = self.classify(training_example);
        classification == training_example.is_positive
    }

    /// Return the most minimal generalization that is consistent with the new training example
    pub fn generalize(&self, training_example: &TrainingExample) -> Option<Self> {
        let attributes = self
            .attributes
            .iter()
            .zip(training_example.attributes.iter())
            .map(|(attribute, other_attribute)| attribute.generalize(other_attribute).unwrap_or(dbg!(attribute.clone())))
            .collect::<Vec<Attribute>>();
        Some(Self {
            attributes,
            dataset_metadata: self.dataset_metadata,
        })
    }

    /// Return the most minimal specialization that is consistent with both hypotheses
    pub fn specialize(
        &self,
        training_example: &TrainingExample,
        value_data: &[Vec<String>],
    ) -> Vec<Self> {
        let mut hypotheses = vec![];
        for (index, (attribute, other_attribute)) in self
            .attributes
            .iter()
            .zip(training_example.attributes.iter())
            .enumerate()
        {
            if let Some(specializations) =
                attribute.specialize(other_attribute, value_data[index].as_slice())
            {
                for specialization in specializations.into_iter() {
                    let mut new_attributes = self.attributes.clone();
                    new_attributes[index] = specialization;
                    hypotheses.push(Hypothesis {
                        attributes: new_attributes,
                        dataset_metadata: self.dataset_metadata,
                    })
                }
            }
        }

        hypotheses
    }

    pub fn to_vec(self) -> Vec<String> {
        // Convert each of the attributes from Attribute type to a string
        self.attributes
            .into_iter()
            .map(|attribute| attribute.to_string())
            .collect()
    }

    pub fn len(&self) -> usize {
        self.attributes.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn from_str(
        record: &str,
        dataset_metadata: &'static DatasetMetadata,
    ) -> Result<Self, ParseBoolError> {
        let record: Vec<&str> = record.split(',').map(|str| str.trim()).collect();
        let attributes: Vec<Attribute> = record
            .iter()
            .enumerate()
            .map(|(index, record)| Attribute::new(record, index, dataset_metadata)) // This unwrap is safe - all cases covered in Attribute enum
            .collect();

        Ok(Hypothesis {
            attributes,
            dataset_metadata,
        })
    }
}

impl Display for Hypothesis<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let attributes = self
            .attributes
            .iter()
            .enumerate()
            .map(|(index, attribute)| attribute.to_valued_string(index, self.dataset_metadata))
            .collect::<Vec<String>>()
            .join(", ");

        f.write_str(&format!("⟨{attributes}⟩"))
    }
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;

    use super::*;

    lazy_static! {
        static ref DATASET_METADATA: DatasetMetadata = {
            serde_yaml::from_reader(std::fs::File::open("./data/enjoysport/metadata.yaml").unwrap())
                .unwrap()
        };
    }

    #[test]
    fn test_row_to_string() {
        let hypothesis = Hypothesis {
            attributes: vec![Attribute::NoValue, Attribute::Any, Attribute::Value(0)],
            dataset_metadata: &DATASET_METADATA,
        };

        let expected = "⟨∅, ?, Foo⟩";
        assert_eq!(expected, hypothesis.to_string());
    }

    #[test]
    fn test_hypothesis_generality_book_example() {
        let hypothesis = Hypothesis::from_str("?,?,Normal,?,?,?", &DATASET_METADATA).unwrap();
        let other =
            Hypothesis::from_str("Sunny,Warm,?,Strong,Warm,Same", &DATASET_METADATA).unwrap();
        assert!(!hypothesis.is_more_general(&other));
    }

    #[test]
    fn test_hypothesis_generality_book_example_2() {
        let hypothesis = Hypothesis::from_str("Sunny,?,?,?,?,?", &DATASET_METADATA).unwrap();
        let other =
            Hypothesis::from_str("Sunny,Warm,?,Strong,Warm,Same", &DATASET_METADATA).unwrap();
        assert!(hypothesis.is_more_general(&other));
    }

    #[test]
    fn test_hypothesis_generality_book_example_3() {
        let hypothesis = Hypothesis::from_str("?,?,?,Weak,?,?", &DATASET_METADATA).unwrap();
        let other =
            Hypothesis::from_str("Sunny,Warm,?,Strong,Warm,Same", &DATASET_METADATA).unwrap();
        assert!(!hypothesis.is_more_general(&other));
    }

    #[test]
    fn test_general_hypothesis() {
        let general = Hypothesis::general(5, &DATASET_METADATA);
        assert_eq!(
            general,
            Hypothesis {
                attributes: vec![Attribute::Any; 5],
                dataset_metadata: &DATASET_METADATA
            }
        );
    }

    #[test]
    fn test_specific_hypothesis() {
        let general = Hypothesis::specific(5, &DATASET_METADATA);
        assert_eq!(
            general,
            Hypothesis {
                attributes: vec![Attribute::NoValue; 5],
                dataset_metadata: &DATASET_METADATA
            }
        );
    }

    #[test]
    fn test_hypothesis_classify_valid() {
        let hypothesis = Hypothesis::from_str("Sunny,?,High,?,Warm,?", &DATASET_METADATA).unwrap();
        let valid = TrainingExample::new(
            &[
                Attribute::Value(0),
                Attribute::Any,
                Attribute::Value(1),
                Attribute::Any,
                Attribute::Value(0),
                Attribute::Any,
            ],
            true,
        );

        assert!(hypothesis.classify(&valid));
    }

    #[test]
    fn test_hypothesis_classify_invalid() {
        let hypothesis = Hypothesis::from_str("Sunny,?,High,?,Warm,?", &DATASET_METADATA).unwrap();
        let invalid = TrainingExample::new(
            &[
                // Should fail, too general
                Attribute::Any,
                Attribute::Any,
                Attribute::Any,
                Attribute::Any,
                Attribute::Any,
                Attribute::Any,
            ],
            true,
        );
        assert!(!hypothesis.classify(&invalid));
    }

    #[test]
    fn test_hypothesis_specialize() {
        let hypothesis = Hypothesis::from_str("?,?,?,?,?,?", &DATASET_METADATA).unwrap();
        let specializer = TrainingExample::from_str(
            "Rainy,Cold,High,Strong,Warm,Change,false",
            &DATASET_METADATA,
        )
        .unwrap();

        let specialized_hypotheses = vec![
            Hypothesis::from_str("Sunny,?,?,?,?,?", &DATASET_METADATA).unwrap(),
            Hypothesis::from_str("?,Warm,?,?,?,?", &DATASET_METADATA).unwrap(),
            Hypothesis::from_str("?,?,Normal,?,?,?", &DATASET_METADATA).unwrap(),
            Hypothesis::from_str("?,?,?,Weak,?,?", &DATASET_METADATA).unwrap(),
            Hypothesis::from_str("?,?,?,?,Cool,?", &DATASET_METADATA).unwrap(),
            Hypothesis::from_str("?,?,?,?,?,Same", &DATASET_METADATA).unwrap(),
        ];

        let value_data = vec![
            vec!["Sunny".to_string(), "Rainy".to_string()],
            vec!["Warm".to_string(), "Cold".to_string()],
            vec!["Normal".to_string(), "High".to_string()],
            vec!["Strong".to_string(), "Weak".to_string()],
            vec!["Warm".to_string(), "Cool".to_string()],
            vec!["Same".to_string(), "Change".to_string()],
        ];

        assert_eq!(
            hypothesis.specialize(&specializer, &value_data),
            specialized_hypotheses
        )
    }

    #[test]
    fn test_is_consistent_book_example() {
        let hypothesis = Hypothesis::from_str("?,?,?,?,?,Same", &DATASET_METADATA).unwrap();
        let training_example =
            TrainingExample::from_str("Sunny,Warm,High,Strong,Cool,Change,true", &DATASET_METADATA)
                .unwrap();
        assert!(!hypothesis.is_consistent(&training_example));
    }
}
