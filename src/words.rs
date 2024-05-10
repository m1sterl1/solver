use crate::matrix::Matrix;
use crate::utils::get_coincidence;
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
        Ok(Self {
            words,
            intersections,
        })
    }

    /// Create new instance from file with words
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let words = read_to_string(path)?;
        let words = words.split('\n');
        Self::from_iter(words)
    }

    pub fn solve(&mut self) {
        self.compute_intersections();
        for i in 0..self.words.len() {
            println!("{} {:?}", self.words[i], self.intersections.row(i))
        }
    }

    pub fn words(&self) -> &Vec<String> {
        &self.words
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
            "processor",
            "durasteel",
            "consisted",
            "extremely",
            "beginning",
            "untouched",
        ];
        let mut words = Words::from_iter(words.iter()).unwrap();
        words.compute_intersections();
        assert_eq!(words.intersections().row(0), vec![&-1, &0, &1, &1, &0, &0]);
    }
}
