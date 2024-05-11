use crate::matrix::Matrix;

use crate::tree::{Node, Tree};
use crate::utils::{get_coincidence, Groups};
use crate::Result;
use std::{fs::read_to_string, path::Path};

pub struct Words {
    words: Vec<String>,
    // matrix with number of intersections between words
    intersections: Matrix<i32>,
}

impl Words {
    /// Create new instance from iterator over items
    /// convertable to String
    /// Check if word's list not empty and words have equal length
    pub fn from_iter<S, I>(iterator: I) -> Result<Self>
    where
        S: ToString,
        I: Iterator<Item = S>,
    {
        let words: Vec<String> = iterator
            .map(|s| s.to_string().trim().into())
            .filter(|s: &String| !s.is_empty())
            .collect();
        Self::check_words(&words)?;
        let intersections = Matrix::new_with(words.len(), words.len(), -1);
        let mut words = Self {
            words,
            intersections,
        };
        words.compute_intersections();
        Ok(words)
    }

    /// Create new instance from file with words
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let words = read_to_string(path)?;
        let words = words.split('\n');
        Self::from_iter(words)
    }

    /// Compute metrics for each word
    ///
    pub fn build_tree(&self) -> Tree {
        let nodes = self.build_children();
        let mut root = Node::new_root();
        root.set_children(nodes);
        Tree::new(root)
    }

    fn build_children(&self) -> Vec<Node> {
        let mut nodes = Vec::new();
        for (i, word) in self.words.iter().enumerate() {
            // create groups of intersections
            // {intercection_number:[word_index1, word_index2,...]}
            let mut groups = self.intersections.row(i).into_iter().copied().groups();
            // remove extra element
            groups.groups_mut().remove(&-1);
            // create Word node
            let mut word_node = Node::new_word(word);
            // iterate intersections/ words
            for (intersections, words) in groups.groups().iter() {
                // convert words indexes to words strings
                let words: Vec<_> = words.iter().map(|&i| &self.words[i]).collect();
                // create new intercection node (Terminal determine intersections)
                let mut intercection_node = Node::new_intersection(*intersections as usize);
                match words.len() {
                    1 => {
                        // if intercection determins single word
                        let child = Node::new_word(words[0]);
                        intercection_node.append_child(child);
                    }
                    _ => {
                        // more than one word
                        let words = Words::from_iter(words.iter()).unwrap();
                        let children = words.build_children();
                        intercection_node.set_children(children)
                    }
                }
                word_node.append_child(intercection_node)
            }
            nodes.push(word_node)
        }
        nodes
    }

    // let first_metric = metrics.first().unwrap();

    pub fn words(&self) -> &Vec<String> {
        &self.words
    }

    pub fn word(&self, i: usize) -> Option<&str> {
        self.words.get(i).map(|s| s.as_str())
    }

    pub fn intersections(&self) -> &Matrix<i32> {
        &self.intersections
    }

    /// Performs checks on words
    fn check_words(words: &[String]) -> Result<()> {
        if words.is_empty() {
            return Err("Empty word's list".into());
        }

        if !Self::is_equal_length(words) {
            return Err("Word's length is not equal".into());
        }
        Ok(())
    }

    /// Check if all words with equal length
    fn is_equal_length(words: &[String]) -> bool {
        let length = words.first().unwrap().len();
        words.iter().all(|w| w.len() == length)
    }

    /// Fill intersections's matrix with result of
    /// get_coincidence function
    fn compute_intersections(&mut self) {
        let words = self.words.clone();
        let f = |(i, j)| {
            let s1 = &words[i] as &str;
            let s2 = &words[j] as &str;
            get_coincidence(s1, s2) as i32
        };
        self.intersections.fill_diag(f);
    }
}

#[cfg(test)]
mod test {
    use super::Words;

    #[test]
    fn test_compute_intersections() {
        let words = [
            "truly", "spare", "feral", "sixty", "whole", "james", "wakes",
        ];
        let words = Words::from_iter(words.iter()).unwrap();
        let mut tree = words.build_tree();
        tree.run();
    }
}
