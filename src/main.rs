#[derive(Debug, Clone)]
pub enum Attribute {
    Left,
    Right,
    NoValue,
    Any,
}

#[derive(Debug, Clone)]
pub struct Hypothesis {
    pub attributes: Vec<Attribute>,
    output: bool,
}

impl Hypothesis {
    pub fn new(attributes: Vec<Attribute>, output: bool) -> Self {
        Self { attributes, output }
    }

    pub fn specific(attribute_count: usize) -> Self {
        Self {
            attributes: vec![Attribute::NoValue],
            output: true,
        }
    }

    pub fn general(attribute_count: usize) -> Self {
        Self {
            attributes: vec![Attribute::Any; attribute_count],
            output: true,
        }
    }

    pub fn is_positive(&self) -> bool {
        self.output
    }

    pub fn is_consistent(&self, other: &Hypothesis) -> bool {
        todo!();
    }
}

fn main() {
    let mut specific_hypotheses = vec![Hypothesis::specific(5)];
    let mut general_hypotheses = vec![Hypothesis::general(5)];

    let examples: Vec<Hypothesis> = vec![];

    examples.into_iter().for_each(|example| {
        if example.is_positive() {
            general_hypotheses.retain(|hypothesis| hypothesis.is_consistent(&example));

            for hypothesis in general_hypotheses.iter() {
                if !hypothesis.is_consistent(&example) {
                }
            }
            // PERF: Inefficient clone - cloning just to use a subset
            let removed: Vec<Hypothesis> = specific_hypotheses
                .clone()
                .into_iter()
                .filter(|hypothesis| !hypothesis.is_consistent(&example))
                .collect();
        } else {
            specific_hypotheses.retain(|hypothesis| hypothesis.is_consistent(&example));
        }
    })
}
