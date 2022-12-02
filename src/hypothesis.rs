use std::fmt::Display;
use std::str::{FromStr, ParseBoolError};

use crate::attribute::Attribute;
use crate::training_example::TrainingExample;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hypothesis {
    pub attributes: Vec<Attribute>,
}

// FIXME: Should hypothesis be renamed to training example? Actual hypotheses all seem to be true
impl Hypothesis {
    pub fn general(n_attributes: usize) -> Self {
        Self {
            attributes: vec![Attribute::Any; n_attributes],
        }
    }

    pub fn specific(n_attributes: usize) -> Self {
        Self {
            attributes: vec![Attribute::NoValue; n_attributes],
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
            .map(|(attribute, other_attribute)| attribute.generalize(other_attribute))
            .collect::<Option<Vec<Attribute>>>()?;
        Some(Self { attributes })
    }

    /// Return the most minimal specialization that is consistent with both hypotheses
    pub fn specialize(
        &self,
        training_example: &TrainingExample,
        value_data: &[Vec<String>],
    ) -> Option<Vec<Self>> {
        let mut hypotheses = vec![];
        // FIXME: RESUME HERE
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
                    })
                }
            }
        }

        Some(hypotheses)
    }

    pub fn to_vec(self) -> Vec<String> {
        // Convert each of the attributes from Attribute type to a string
        self.attributes
            .into_iter()
            .map(|attribute| attribute.to_string())
            .collect()
    }
}

impl Display for Hypothesis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let attributes = self
            .attributes
            .iter()
            .map(|h| h.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        f.write_str(&format!("⟨{attributes}⟩"))
    }
}

impl FromStr for Hypothesis {
    type Err = ParseBoolError;

    fn from_str(record: &str) -> Result<Self, Self::Err> {
        let record: Vec<&str> = record.split(',').map(|str| str.trim()).collect();
        let attributes: Vec<Attribute> = record
            .iter()
            .map(|record| Attribute::from_str(record).unwrap()) // This unwrap is safe - all cases covered in Attribute enum
            .collect();

        Ok(Hypothesis { attributes })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_row_to_string() {
        let hypothesis = Hypothesis {
            attributes: vec![
                Attribute::NoValue,
                Attribute::Any,
                Attribute::Value("Foo".to_string()),
            ],
        };

        let expected = "⟨∅, ?, Foo⟩";
        assert_eq!(expected, hypothesis.to_string());
    }

    #[test]
    fn test_hypothesis_generality_book_example() {
        let hypothesis = Hypothesis::from_str("?,?,Normal,?,?,?").unwrap();
        let other = Hypothesis::from_str("Sunny,Warm,?,Strong,Warm,Same").unwrap();
        assert!(!hypothesis.is_more_general(&other));
    }

    #[test]
    fn test_hypothesis_generality_book_example_2() {
        let hypothesis = Hypothesis::from_str("Sunny,?,?,?,?,?").unwrap();
        let other = Hypothesis::from_str("Sunny,Warm,?,Strong,Warm,Same").unwrap();
        assert!(hypothesis.is_more_general(&other));
    }

    #[test]
    fn test_hypothesis_generality_book_example_3() {
        let hypothesis = Hypothesis::from_str("?,?,?,Weak,?,?").unwrap();
        let other = Hypothesis::from_str("Sunny,Warm,?,Strong,Warm,Same").unwrap();
        assert!(!hypothesis.is_more_general(&other));
    }

    #[test]
    fn test_general_hypothesis() {
        let general = Hypothesis::general(5);
        assert_eq!(
            general,
            Hypothesis {
                attributes: vec![Attribute::Any; 5],
            }
        );
    }

    #[test]
    fn test_specific_hypothesis() {
        let general = Hypothesis::specific(5);
        assert_eq!(
            general,
            Hypothesis {
                attributes: vec![Attribute::NoValue; 5],
            }
        );
    }

    #[test]
    fn test_hypothesis_classify_valid() {
        let hypothesis = Hypothesis::from_str("Foo,?,Bar,?,Baz").unwrap();
        let valid = TrainingExample::new(
            &[
                Attribute::Value("Foo".into()),
                Attribute::Value("Baz".into()),
                Attribute::Value("Bar".into()),
                Attribute::Any,
                Attribute::Value("Baz".into()),
            ],
            true,
        );

        assert!(hypothesis.classify(&valid));
    }

    #[test]
    fn test_hypothesis_classify_invalid() {
        let hypothesis = Hypothesis::from_str("Foo,?,Bar,?,Baz").unwrap();
        let invalid = TrainingExample::new(
            &[
                Attribute::Value("Foo".into()),
                Attribute::Any,
                Attribute::Any, // Should fail, any is too general here
                Attribute::Any,
                Attribute::Value("Baz".into()),
            ],
            true,
        );
        assert!(!hypothesis.classify(&invalid));
    }

    #[test]
    fn test_hypothesis_specialize() {
        let hypothesis = Hypothesis::from_str("?,?,?,?,?,?").unwrap();
        let specializer =
            TrainingExample::from_str("Rainy,Cold,High,Strong,Warm,Change,false").unwrap();

        let specialized_hypotheses = vec![
            Hypothesis::from_str("Sunny,?,?,?,?,?").unwrap(),
            Hypothesis::from_str("?,Warm,?,?,?,?").unwrap(),
            Hypothesis::from_str("?,?,Normal,?,?,?").unwrap(),
            Hypothesis::from_str("?,?,?,Weak,?,?").unwrap(),
            Hypothesis::from_str("?,?,?,?,Cool,?").unwrap(),
            Hypothesis::from_str("?,?,?,?,?,Same").unwrap(),
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
            dbg!(hypothesis.specialize(&specializer, &value_data).unwrap()),
            specialized_hypotheses
        )
    }

    #[test]
    fn test_is_consistent_book_example() {
        let hypothesis = Hypothesis::from_str("?,?,?,?,?,Same").unwrap();
        let training_example =
            TrainingExample::from_str("Sunny,Warm,High,Strong,Cool,Change,true").unwrap();
        assert!(!hypothesis.is_consistent(&training_example));
    }
}
