use std::convert::Infallible;
use std::fmt::Display;
use std::str::FromStr;

/// Deriving `PartialOrd` works by ranking enums in the order they are defined (Any is ranked the
/// highest - i.e. it is the most general)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Attribute {
    /// No value can yield a positive example
    NoValue,

    /// This specific value can lead a positive example
    Value(
        /// The specific value that can lead to a positive example
        String,
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
            Attribute::Value(val) => f.write_str(val),
        }
    }
}

impl Attribute {
    pub fn is_consistent(&self, other: &Self) -> bool {
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
    pub fn generalize(&self, other: &Self) -> Option<Self> {
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
    pub fn specialize(&self, other: &Self) -> Option<Vec<Self>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_attributes() {
        let most_general = Attribute::Any;
        let middle = Attribute::Value("Foo".to_string());
        let most_specific = Attribute::NoValue;

        assert!(most_general > middle);
        assert!(most_general > most_specific);
        assert!(middle > most_specific);
    }
}
