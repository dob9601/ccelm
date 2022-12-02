use std::fmt::Display;
use std::str::{FromStr, ParseBoolError};

use csv::StringRecord;

use crate::attribute::Attribute;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrainingExample {
    pub attributes: Vec<Attribute>,
    pub is_positive: bool,
}

// FIXME: Should hypothesis be renamed to training example? Actual hypotheses all seem to be true
impl TrainingExample {
    pub fn new(attributes: &[Attribute], is_positive: bool) -> Self {
        Self {
            attributes: attributes.to_vec(),
            is_positive,
        }
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

impl Display for TrainingExample {
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

impl TryFrom<StringRecord> for TrainingExample {
    type Error = ParseBoolError;

    fn try_from(record: StringRecord) -> Result<Self, Self::Error> {
        let attributes: Vec<Attribute> = record
            .iter()
            .take(record.len() - 1)
            .map(|record| Attribute::from_str(record).unwrap()) // This unwrap is safe - all cases covered in Attribute enum
            .collect();

        Ok(TrainingExample {
            attributes,
            is_positive: bool::from_str(record.get(record.len() - 1).unwrap())?,
        })
    }
}

impl FromStr for TrainingExample {
    type Err = ParseBoolError;

    fn from_str(record: &str) -> Result<Self, Self::Err> {
        let record: Vec<&str> = record.split(',').map(|str| str.trim()).collect();
        let attributes: Vec<Attribute> = record[..record.len() - 1]
            .iter()
            .map(|record| Attribute::from_str(record).unwrap()) // This unwrap is safe - all cases covered in Attribute enum
            .collect();

        let is_positive = bool::from_str(record.last().unwrap())?;

        Ok(TrainingExample {
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

            let row = TrainingExample::try_from(result).unwrap();
            assert_eq!(
                row,
                TrainingExample {
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
        let row = TrainingExample {
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
