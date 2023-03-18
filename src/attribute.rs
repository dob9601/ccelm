use std::fmt::Display;

use crate::reader::DatasetMetadata;

/// Deriving `PartialOrd` works by ranking enums in the order they are defined (Any is ranked the
/// highest - i.e. it is the most general)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Attribute {
    /// No value can yield a positive example
    NoValue,

    /// This specific value can lead a positive example
    Value(
        /// The specific value that can lead to a positive example
        u8,
    ),

    /// Any value of this attribute can yield a positive example
    Any,
}

/// Convert this Attribute to string, rendering it as per the notation used in Machine Learning
/// (Tom Mitchell)
impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Attribute::NoValue => f.write_str("∅"),
            Attribute::Any => f.write_str("?"),
            Attribute::Value(val) => f.write_str(&val.to_string()),
        }
    }
}

impl Attribute {
    pub fn new(attribute: &str, column_index: usize, dataset_metadata: &DatasetMetadata) -> Self {
        if attribute == dataset_metadata.no_value_string {
            Self::NoValue
        } else if attribute == dataset_metadata.any_value_string {
            Self::Any
        } else {
            Self::Value(Attribute::value_to_index(
                attribute,
                column_index,
                dataset_metadata,
            ))
        }
    }

    fn value_to_index(value: &str, column_index: usize, dataset_metadata: &DatasetMetadata) -> u8 {
        dataset_metadata.columns[column_index]
            .iter()
            .position(|item| item == value)
            .unwrap()
            .try_into()
            .unwrap()
    }

    pub fn to_valued_string(&self, column_index: usize, dataset_metadata: &DatasetMetadata) -> String {
        if let Self::Value(value) = self {
            dataset_metadata.columns[column_index][usize::from(*value)].clone()
        } else {
            self.to_string()
        }
    }

    pub fn is_consistent(&self, other: &Self) -> bool {
        if let (Attribute::Value(left), Attribute::Value(right)) = (self, other) {
            return left == right;
        }
        self >= other
    }

    /// Return the most specific attribute that satisfies other whilst being the smallest
    /// generalization of self.
    ///
    /// # Returns
    /// Some attribute if the attribute can be further generalized by the provided attribute. Else,
    /// none
    pub fn generalize(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (a, b) if a == b => None,
            (Attribute::NoValue, Attribute::Any) => Some(Attribute::Any),
            (Attribute::NoValue, Attribute::Value(v)) => Some(Attribute::Value(*v)),

            (Attribute::Any, _) => None,

            (Attribute::Value(_), Attribute::NoValue) => None,
            (Attribute::Value(_), Attribute::Any) => Some(Attribute::Any),
            (Attribute::Value(_), Attribute::Value(_)) => Some(Attribute::Any), // Only if values
            // aren't equal to each other
            _ => unreachable!(),
        }
    }

    /// Return the most specific attribute that satisfies other whilst being the smallest
    /// generalization of self. Only works if the training example is negative.
    ///
    /// Some attribute if the attribute can be further specialized by the provided attribute. Else,
    /// none
    pub fn specialize(&self, other: &Self, possible_values: &[String]) -> Option<Vec<Self>> {
        match (self, other) {
            (a, b) if a == b => None,
            (Attribute::NoValue, _) => None, // NoValue cannot be specialized further
            (Attribute::Any, Attribute::NoValue) => Some(vec![Attribute::NoValue]),
            (Attribute::Any, Attribute::Value(v)) => {
                let values: Vec<Attribute> = (0..possible_values.len())
                    .map(|index| u8::try_from(index).unwrap())
                    .filter(|index| index != v)
                    .map(Attribute::Value)
                    .collect();

                if values.is_empty() {
                    Some(vec![Attribute::NoValue])
                } else {
                    Some(values)
                }
            }
            (Attribute::Value(_), Attribute::NoValue) => Some(vec![Attribute::NoValue]),
            (Attribute::Value(_), Attribute::Any) => None,
            (Attribute::Value(_), Attribute::Value(_)) => None,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_attributes() {
        let most_general = Attribute::Any;
        let middle = Attribute::Value(1);
        let most_specific = Attribute::NoValue;

        assert!(most_general > middle);
        assert!(most_general > most_specific);
        assert!(middle > most_specific);
    }

    #[test]
    fn test_is_consistent_any_with_value() {
        let any = Attribute::Any;
        let value = Attribute::Value(1);

        assert!(any.is_consistent(&value));
    }

    #[test]
    fn test_not_is_consistent_value_with_other_value() {
        let value = Attribute::Value(1);
        let other_value = Attribute::Value(2);

        assert!(!value.is_consistent(&other_value));
    }

    #[test]
    fn test_is_consistent_value_reflexive() {
        let value = Attribute::Value(1);

        assert!(value.is_consistent(&value));
    }
}
