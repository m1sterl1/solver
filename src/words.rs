use std::{fs::read_to_string, path::Path};
use crate::matrix::Matrix;
use crate::utils::{get_coincidence,Groups, GroupsStruct};
use crate::metric::Metric;
use crate::Result;

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

    /// Compute intersections, metrics for each word
    /// 
    pub fn solve(&mut self) -> Vec<(usize, Metric)>{
        self.compute_intersections();
        // form Vec<(index, metric)>
        let mut metrics = (0..self.intersections.rows())
        .map(|i|self.intersections.row(i))
        .map(|r|r.into_iter().map(|i|*i).collect::<Vec<_>>())
        .map(|r|r.into_iter().groups())
        .map(|mut g|Metric::from_group(g))
        .enumerate()
        .collect::<Vec<_>>();
        // take all equal highest metrics 
        metrics.sort_by(|a, b|b.1.partial_cmp(&a.1).unwrap());
        let first_metric = &metrics.first().unwrap().1.clone();
        metrics
        .into_iter()
        .filter(|(i,m)|m == first_metric)
        .collect()
    }

        // let first_metric = metrics.first().unwrap();


    pub fn words(&self) -> &Vec<String> {
        &self.words
    }

    pub fn word(&self, i:usize) -> Option<&str>{
        self.words.get(i).map(|s|s.as_str())
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
        // let words = [
        //     "endure",
        //     "period",
        //     "around",
        //     "praise",
        //     "lovely",
        //     "skulls",
        //     "almost",
        // ]; praise -> period
        let words = [
            "working",
            "annoyed",
            "essence",
            "watched",
            "harmful",
            "primate",
            "caravan",
        ];
        // let words = [
        //     "working",
        //     "harmful", //
        //     "caravan", //
        // ];
        // let words = [
        //     "working", //
        //     "watched", //
        //     "primate",
        // ];
        // let words = [
        //     "annoyed",
        //     "essence", //
        //     "primate", //
        // ];
        let mut words = Words::from_iter(words.iter()).unwrap();
        // assert_eq!(words.intersections().row(0), vec![&-1, &0, &1, &1, &0, &0]);
        let metrics = words.solve();
        for (i,w) in words.words().iter().enumerate(){
            println!("{i}.\t{w}");
        }
        for (i, m) in metrics{
            println!("{}\t{m}", &words.word(i).unwrap())
        }
    }
}
