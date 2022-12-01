use std::cmp::Ordering;
use std::fmt::Display;
use std::str::{FromStr, ParseBoolError};

use csv::StringRecord;

use crate::attribute::Attribute;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hypothesis {
    pub attributes: Vec<Attribute>,
    pub is_positive: bool,
}

impl Hypothesis {
    pub fn general(n_attributes: usize) -> Self {
        Self {
            attributes: vec![Attribute::Any; n_attributes],
            is_positive: true,
        }
    }

    pub fn specific(n_attributes: usize) -> Self {
        Self {
            attributes: vec![Attribute::NoValue; n_attributes],
            is_positive: true,
        }
    }

    pub fn is_consistent(&self, other: &Self) -> bool {
        debug_assert_eq!(self.attributes.len(), other.attributes.len());

        if self.is_positive != other.is_positive {
            return true;
        }

        for (attribute, other_attribute) in self.attributes.iter().zip(other.attributes.iter()) {
            // This condition needs to take into account the output - a negative example shouldn't
            // invalidate all positive examples as it currently does.
            // FIXME:
            if !attribute.is_consistent(other_attribute) {
                return false;
            }
        }
        true
    }

    /// Return the most minimal generalization that is consistent with both hypotheses
    pub fn generalize(&self, other: &Self) -> Option<Self> {
        let attributes = self
            .attributes
            .iter()
            .zip(other.attributes.iter())
            .map(|(attribute, other_attribute)| attribute.generalize(other_attribute))
            .collect::<Option<Vec<Attribute>>>()?;
        Some(Self {
            attributes,
            is_positive: self.is_positive,
        })
    }

    /// Return the most minimal specialization that is consistent with both hypotheses
    pub fn specialize(&self, other: &Self) -> Option<Vec<Self>> {
        let mut hypotheses = vec![];
        // FIXME: RESUME HERE
        for (index, (attribute, other_attribute)) in self
            .attributes
            .iter()
            .zip(other.attributes.iter())
            .enumerate()
        {
            if let Some(specializations) = attribute.specialize(other_attribute) {
                for specialization in specializations.into_iter() {
                    let mut new_attributes = self.attributes.clone();
                    new_attributes[index] = specialization;
                    hypotheses.push(Hypothesis {
                        attributes: new_attributes,
                        is_positive: self.is_positive,
                    })
                }
            }
        }

        Some(hypotheses)
    }

    pub fn to_vec(self) -> Vec<String> {
        // Convert each of the attributes from Attribute type to a string
        let mut bytes: Vec<String> = self
            .attributes
            .into_iter()
            .map(|attribute| attribute.to_string())
            .collect();

        bytes.push(self.is_positive.to_string());

        bytes
    }
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
impl PartialOrd for Hypothesis {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let orderings: Vec<Ordering> = self
            .attributes
            .iter()
            .zip(other.attributes.iter())
            .map(|(attribute, other_attribute)| attribute.cmp(other_attribute))
            .collect();

        if orderings.iter().all(|ordering| ordering.is_ge()) {
            Some(Ordering::Greater)
        } else if orderings.iter().all(|ordering| ordering.is_le()) {
            Some(Ordering::Less)
        } else {
            None
        }
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

        let output = if self.is_positive { "Yes" } else { "No" };

        f.write_str(&format!("⟨{attributes}⟩ => {output}"))
    }
}

impl TryFrom<StringRecord> for Hypothesis {
    type Error = ParseBoolError;

    fn try_from(record: StringRecord) -> Result<Self, Self::Error> {
        let attributes: Vec<Attribute> = record
            .iter()
            .take(record.len() - 1)
            .map(|record| Attribute::from_str(record).unwrap()) // This unwrap is safe - all cases covered in Attribute enum
            .collect();

        Ok(Hypothesis {
            attributes,
            is_positive: bool::from_str(record.get(record.len() - 1).unwrap())?,
        })
    }
}

impl FromStr for Hypothesis {
    type Err = ParseBoolError;

    fn from_str(record: &str) -> Result<Self, Self::Err> {
        let record: Vec<&str> = record.split(',').map(|str| str.trim()).collect();
        let attributes: Vec<Attribute> = record[..record.len() - 1]
            .iter()
            .map(|record| Attribute::from_str(record).unwrap()) // This unwrap is safe - all cases covered in Attribute enum
            .collect();

        let is_positive = bool::from_str(record.last().unwrap())?;

        Ok(Hypothesis {
            attributes,
            is_positive,
        })
    }
}

#[cfg(test)]
mod tests {
    use csv::Reader;

    use super::*;

    #[test]
    fn test_row_deserialization() {
        let data = "\
Col1,Col2,Col3,Col4,Col5,Col6,output
∅,?,SomeValue,SomeOtherValue,∅,?,true
";
        let mut reader = Reader::from_reader(data.as_bytes());
        for result in reader.records() {
            let result = result.expect("Deserialization to be successful");

            let row = Hypothesis::try_from(result).unwrap();
            assert_eq!(
                row,
                Hypothesis {
                    attributes: vec![
                        Attribute::NoValue,
                        Attribute::Any,
                        Attribute::Value("SomeValue".to_string()),
                        Attribute::Value("SomeOtherValue".to_string()),
                        Attribute::NoValue,
                        Attribute::Any,
                    ],
                    is_positive: true,
                }
            )
        }
    }

    #[test]
    fn test_row_serialization() {
        let row = Hypothesis {
            attributes: vec![
                Attribute::NoValue,
                Attribute::Any,
                Attribute::Value("Foo".to_string()),
            ],
            is_positive: true,
        };
        let mut writer = csv::Writer::from_writer(vec![]);
        writer.write_record(row.to_vec()).unwrap();

        let expected = "∅,?,Foo,true\n";
        let actual = std::str::from_utf8(&writer.into_inner().unwrap())
            .unwrap()
            .to_string();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_hypothesis_generality_extremes() {
        let most_general = Hypothesis::from_str("?,?,?,?,?,true").unwrap();
        let most_specific = Hypothesis::from_str("∅,∅,∅,∅,∅,true").unwrap();
        assert!(most_general > most_specific);
    }

    #[test]
    fn test_hypothesis_generality() {
        let specific = Hypothesis::from_str("Foo,?,Bar,?,Baz,true").unwrap();
        let general = Hypothesis::from_str("Foo,?,?,?,Baz,true").unwrap();
        assert!(general > specific);
    }

    #[test]
    fn test_hypothesis_generality_no_ordering() {
        let specific = Hypothesis::from_str("Foo,?,Bar,?,Baz,true").unwrap();

        // Edge case, 3rd attribute is more general - 5th is more specific
        let general = Hypothesis::from_str("Foo,?,?,?,∅,true").unwrap();

        // Neither hypothesis is more or less general in this case
        assert!(general.partial_cmp(&specific).is_none());
    }
}
