use ccelm::hypothesis::Hypothesis;

fn main() {
    let mut specific_hypotheses = vec![Hypothesis::specific(5)];
    let mut general_hypotheses = vec![Hypothesis::general(5)];

    let examples: Vec<Hypothesis> = vec![];

    for example in examples.into_iter() {
        if example.is_positive {
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
    }
}
