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
pub struct Row {
    pub attributes: Vec<Attribute>,
    pub output: bool,
}

impl Row {
    pub fn to_vec(self) -> Vec<String> {
        // Convert each of the attributes from Attribute type to a string
        let mut bytes: Vec<String> = self
            .attributes
            .into_iter()
            .map(|attribute| attribute.to_string())
            .collect();

        bytes.push(self.output.to_string());

        bytes
    }
}

impl TryFrom<StringRecord> for Row {
    type Error = ParseBoolError;

    fn try_from(record: StringRecord) -> Result<Self, Self::Error> {
        let attributes: Vec<Attribute> = record
            .iter()
            .take(record.len() - 1)
            .map(|record| Attribute::from_str(record).unwrap()) // This unwrap is safe - all cases covered in Attribute enum
            .collect();

        Ok(Row {
            attributes,
            output: bool::from_str(record.get(record.len() - 1).unwrap())?,
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

            let row = Row::try_from(result).unwrap();
            assert_eq!(
                row,
                Row {
                    attributes: vec![
                        Attribute::NoValue,
                        Attribute::Any,
                        Attribute::Value("SomeValue".to_string()),
                        Attribute::Value("SomeOtherValue".to_string()),
                        Attribute::NoValue,
                        Attribute::Any,
                    ],
                    output: true,
                }
            )
        }
    }

    #[test]
    fn test_row_serialization() {
        let row = Row {
            attributes: vec![
                Attribute::NoValue,
                Attribute::Any,
                Attribute::Value("Foo".to_string()),
            ],
            output: true,
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
