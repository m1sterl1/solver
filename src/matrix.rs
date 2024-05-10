use std::{
    fmt::{Display, Formatter},
    fs::read_to_string,
    ops::{Index, IndexMut},
    path::Path,
};

use crate::Result;


pub struct Matrix<T> {
    m: usize, // number of rows
    n: usize, // number of cols
    inner: Vec<T>,
}

impl<T: Copy+Default> Matrix<T> {

    /// Create matrix with `m` rows and `n` columns
    /// with default value for type T 
    pub fn new(m: usize, n: usize) -> Self {
        Self::new_with(m, n, Default::default())
    }

    /// Create matrix with `m` rows and `n` columns
    /// with value `val`
    pub fn new_with(m: usize, n: usize, val: T) -> Self {
        let mut inner = Vec::with_capacity(m * n);
        for _ in 0..m * n {
            inner.push(val.clone());
        }
        Self { m, n, inner }
    }

    /// Create matrix with `m` rows and `n` columns
    /// using values provided with iterator
    fn new_iter(mut iter: impl Iterator<Item = T>, m: usize, n: usize) -> Self {
        let mut inner = Vec::with_capacity(m * n);
        for _ in 0..m * n {
            inner.push(iter.next().unwrap());
        }
        Self { m, n, inner }
    }

    /// Fill matrix with values provided with callback
    /// Fn(i,j) -> T
    pub fn fill<F:Fn((usize, usize)) -> T>(&mut self, f:F){
        for i in 0..self.n{
            for j in 0..self.m{
                self[(i,j)] = f((i,j))
            }
        }
    }

    /// Fill diagonal matrix with values provided with callback
    /// Compute values not for all indexes but only 
    /// for left bottom triagonal part
    pub fn fill_diag<F:Fn((usize, usize)) -> T>(&mut self, f:F){
        for i in 0..self.n{
            for j in 0..i{
                self[(i,j)] = f((i,j));
                self[(j,i)] = self[(i,j)];
            }
        }
    }

    /// Returns Vec of ref to values for `j` column
    pub fn column(&self, j: usize) -> Vec<&T> {
        let mut column = Vec::new();
        for i in 0..self.m {
            column.push(&self[(i, j)])
        }
        column
    }

    /// Returns Vec of ref to values for `i` row
    pub fn row(&self, i: usize) -> Vec<&T> {
        self.inner[i * self.n..i * self.n + self.m]
            .iter()
            .collect()
    }
}

impl<T: Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.m {
            for j in 0..self.n {
                write!(f, "{}", self[(i, j)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (i, j) = index;
        &self.inner[i * self.n + j]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (i, j) = index;
        &mut self.inner[i * self.n + j]
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;

    #[test]
    fn test_new_iter() {
        let words = [
            "favored", "thirsty", "whoever", "ghengis", "mounted", "freedom",
        ];
        // Convert words to iterator of chars
        let chars = words.iter().map(|s| s.chars()).collect::<Vec<_>>();
        let chars = chars.into_iter().flatten().collect::<Vec<_>>();
        let m = Matrix::new_iter(chars.into_iter(), 6, 7);

        println!("{m}");

        // Test columnt
        assert_eq!(m.column(0).into_iter().collect::<String>(), "ftwgmf".to_string());

        // Test row
        assert_eq!(m.row(2).into_iter().collect::<String>(), "whoeve".to_string());

    }

    #[test]
    fn test_new() {
        let m: Matrix<usize> = Matrix::new(6, 7);
        println!("{m}");
    }

    #[test]
    fn test_fill(){
        let mut m: Matrix<i32> = Matrix::new(3, 3);
        let f = |(i,j)|(i+j) as i32;
        m.fill(f);
        println!{"{m}"}
    }
}
