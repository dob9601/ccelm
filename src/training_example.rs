use std::fmt::Display;
use std::str::{FromStr, ParseBoolError};

use crate::attribute::Attribute;
use crate::reader::DatasetMetadata;
use crate::Hypothesis;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrainingExample {
    pub attributes: Vec<Attribute>,
    pub is_positive: bool,
}

impl From<Hypothesis<'_>> for TrainingExample {
    fn from(hypothesis: Hypothesis<'_>) -> Self {
        TrainingExample {
            attributes: hypothesis.attributes,
            is_positive: true,
        }
    }
}

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

    pub fn from_str(
        record: &str,
        dataset_metadata: &DatasetMetadata,
    ) -> Result<Self, ParseBoolError> {
        let record: Vec<&str> = record.split(',').map(|str| str.trim()).collect();
        let attributes: Vec<Attribute> = record[..record.len() - 1]
            .iter()
            .enumerate()
            .map(|(index, record)| Attribute::new(record, index, dataset_metadata))
            .collect();

        let is_positive = bool::from_str(record.last().unwrap())?;

        Ok(TrainingExample {
            attributes,
            is_positive,
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_serialization() {
        let row = TrainingExample {
            attributes: vec![Attribute::NoValue, Attribute::Any, Attribute::Value(0)],
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
