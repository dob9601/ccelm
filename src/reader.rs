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
        delimiter: u8,
    ) -> Result<Self, csv::Error> {
        Ok(Self {
            reader: csv::ReaderBuilder::new()
                .delimiter(delimiter)
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
                    .map(|record| {
                        Attribute::new(
                            record,
                            self.metadata.any_value_string.as_deref(),
                            self.metadata.no_value_string.as_deref(),
                        )
                    }) // This unwrap is safe - all cases covered in Attribute enum
                    .collect();
                Some(Ok(TrainingExample::new(&attributes, is_positive)))
            }
            Err(err) => Some(Err(err.into())),
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct DatasetMetadata {
    pub columns: Vec<Vec<String>>,
    pub no_value_string: Option<String>,
    pub any_value_string: Option<String>,
}
