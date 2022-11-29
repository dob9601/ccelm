use std::convert::Infallible;
use std::fmt::Display;
use std::str::{FromStr, ParseBoolError};

use csv::StringRecord;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Attribute {
    NoValue,
    Any,
    Value(String),
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Attribute::NoValue => f.write_str("∅"),
            Attribute::Any => f.write_str("?"),
            Attribute::Value(val) => f.write_str(val),
        }
    }
}

impl Attribute {
    fn is_consistent(&self, other: &Self) -> bool {
        // FIXME: May be wrong depending on knowledge of consistency
        match (self, other) {
            (Attribute::NoValue, Attribute::NoValue) => true,
            (Attribute::NoValue, Attribute::Any) => false,
            (Attribute::NoValue, Attribute::Value(_)) => false,
            (Attribute::Any, Attribute::NoValue) => false,
            (Attribute::Any, Attribute::Any) => true,
            (Attribute::Any, Attribute::Value(_)) => true,
            (Attribute::Value(_), Attribute::NoValue) => false,
            (Attribute::Value(_), Attribute::Any) => true,
            (Attribute::Value(left), Attribute::Value(right)) => left == right,
        }
    }

    /// Return the most specific attribute that satisfies other whilst being the smallest
    /// generalization of self
    fn generalize(&self, other: &Self) -> Option<Self> {
        // FIXME: May be wrong depending on knowledge of consistency
        match (self, other) {
            (a, b) if a == b => Some(a.clone()),
            (Attribute::NoValue, Attribute::Any) => Some(Attribute::Any),
            (Attribute::NoValue, Attribute::Value(v)) => Some(Attribute::Value(v.clone())),
            (Attribute::Any, Attribute::NoValue) => None,
            (Attribute::Any, Attribute::Value(_)) => Some(Attribute::Any),
            (Attribute::Value(_), Attribute::NoValue) => None,
            (Attribute::Value(_), Attribute::Any) => Some(Attribute::Any),
            (a, b) if a != b => Some(Attribute::Any),
            _ => unreachable!(),
        }
    }

    /// Return the most specific attribute that satisfies other whilst being the smallest
    /// generalization of self
    fn specialize(&self, other: &Self) -> Option<Vec<Self>> {
        // FIXME: May be wrong depending on knowledge of consistency
        match (self, other) {
            (a, b) if a == b => Some(vec![a.clone()]),
            (Attribute::NoValue, Attribute::Any) => None,
            (Attribute::NoValue, Attribute::Value(v)) => Some(vec![Attribute::Value(v.clone())]),
            (Attribute::Any, Attribute::NoValue) => Some(vec![Attribute::NoValue]),
            (Attribute::Any, Attribute::Value(v)) => Some(vec![Attribute::Value(v.clone())]),
            (Attribute::Value(_), Attribute::NoValue) => Some(vec![Attribute::NoValue]),
            (Attribute::Value(_), Attribute::Any) => None,
            (Attribute::Value(left), Attribute::Value(right)) if left != right => Some(vec![
                Attribute::Value(left.clone()),
                Attribute::Value(right.clone()),
            ]),
            _ => unreachable!(),
        }
    }
}

impl FromStr for Attribute {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "∅" {
            Ok(Self::NoValue)
        } else if s == "?" {
            Ok(Self::Any)
        } else {
            Ok(Self::Value(s.to_string()))
        }
    }
}

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
        for (index, (attribute, other_attribute)) in self.attributes.iter().zip(other.attributes.iter()).enumerate() {
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

impl Display for Hypothesis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let attributes = self
            .attributes
            .iter()
            .map(|h| h.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        let output = if self.is_positive { "Yes" } else { "No" };

        f.write_str(&format!("{attributes} => {output}"))
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
}
