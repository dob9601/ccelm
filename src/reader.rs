use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;

use csv::StringRecord;
use serde::Deserialize;

use crate::{Attribute, TrainingExample};

pub struct DatasetReader {
    reader: csv::Reader<File>,
    metadata: DatasetMetadata,
}

impl DatasetReader {
    pub fn new<P: AsRef<Path>>(
        dataset_path: P,
        metadata: DatasetMetadata,
    ) -> Result<Self, csv::Error> {
        Ok(Self {
            reader: csv::ReaderBuilder::new()
                .trim(csv::Trim::All)
                .delimiter(metadata.delimiter.try_into().unwrap())
                .from_path(dataset_path)?,
            metadata,
        })
    }

    pub fn attributes(&mut self) -> Result<Vec<&str>, csv::Error> {
        let mut headers = self
            .reader
            .headers()
            .map(|record| record.into_iter().collect::<Vec<_>>())?;

        headers.pop();

        Ok(headers)
    }
}

impl Iterator for DatasetReader {
    type Item = Result<TrainingExample, Box<dyn Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut record = StringRecord::new();

        let did_read = self.reader.read_record(&mut record);

        if let Ok(false) = did_read {
            return None;
        }

        let maybe_is_positive = bool::from_str(record.get(record.len() - 1).unwrap());

        match maybe_is_positive {
            Ok(is_positive) => {
                let attributes: Vec<Attribute> = record
                    .iter()
                    .take(record.len() - 1)
                    .enumerate()
                    .map(|(index, record)| Attribute::new(record, index, &self.metadata)) // This unwrap is safe - all cases covered in Attribute enum
                    .collect();
                Some(Ok(TrainingExample::new(&attributes, is_positive)))
            }
            Err(err) => Some(Err(err.into())),
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct DatasetMetadata {
    pub columns: Vec<Vec<String>>,

    pub delimiter: char,

    #[serde(default = "no_value_string_default")]
    pub no_value_string: String,

    #[serde(default = "any_value_string_default")]
    pub any_value_string: String,
}

fn no_value_string_default() -> String {
    "∅".to_string()
}

fn any_value_string_default() -> String {
    "?".to_string()
}

impl DatasetMetadata {
    pub fn new(
        columns: Vec<Vec<String>>,
        no_value_string: Option<String>,
        any_value_string: Option<String>,
        delimiter: char,
    ) -> Self {
        Self {
            columns,
            no_value_string: no_value_string.unwrap_or_else(no_value_string_default),
            any_value_string: any_value_string.unwrap_or_else(any_value_string_default),
            delimiter,
        }
    }
}
