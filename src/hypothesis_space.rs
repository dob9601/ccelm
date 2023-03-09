use std::collections::HashMap;
use std::fmt::Display;

use crate::{Attribute, Hypothesis};

#[derive(Debug)]
pub struct TreeNode {
    children: HashMap<Attribute, TreeNode>,
}

impl TreeNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
        }
    }

    pub fn insert(&mut self, hypothesis: Hypothesis) {
        self.insert_attribute_chain(hypothesis.attributes)
    }

    pub fn get_hypotheses(self) -> Vec<Hypothesis> {
        self.get_attribute_chains()
            .into_iter()
            .map(|chain| Hypothesis { attributes: chain })
            .collect()
    }

    fn insert_attribute_chain(&mut self, mut attributes: Vec<Attribute>) {
        if attributes.is_empty() {
            return;
        }

        let current_attribute = attributes.remove(0);

        let node = self
            .children
            .entry(current_attribute)
            .or_insert_with(TreeNode::new);

        node.insert_attribute_chain(attributes)
    }

    fn get_attribute_chains(self) -> Vec<Vec<Attribute>> {
        let mut attributes: Vec<Vec<Attribute>> = vec![];

        for (attribute, child_node) in self.children.into_iter() {
            let mut child_attributes = child_node.get_attribute_chains();

            if child_attributes.is_empty() {
                attributes.push(vec![attribute]);
                continue;
            }

            for child_attribute in child_attributes.iter_mut() {
                child_attribute.insert(0, attribute.clone())
            }

            attributes.extend(child_attributes);
        }

        attributes
    }
}

impl Display for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::Hypothesis;

    use super::*;

    #[test]
    fn test_attribute_tree() {
        let mut tree = TreeNode::new();

        let hypothesis_a = Hypothesis::from_str("?,?,?,?,Cool,?").unwrap();
        let hypothesis_b = Hypothesis::from_str("?,?,?,?,?,Same").unwrap();
        tree.insert(hypothesis_a);
        tree.insert(hypothesis_b);

        println!("{:?}", tree.get_hypotheses())
    }
}
