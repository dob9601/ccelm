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

        for (attribute, other_attribute) in self.attributes.iter().zip(other.attributes.iter()) {
            if !attribute.is_consistent(other_attribute) {
                return false;
            }
        }
        true
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
