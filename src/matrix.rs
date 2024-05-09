use std::{ops::Index, fmt::{Formatter, Display, write}};

struct Matrix<T>{
    m: usize,       // number of rows 
    n: usize,       // number of cols 
    inner: Vec<T>,
}

impl<T:Default> Matrix<T>{
    fn new(m:usize, n:usize) -> Self{
        let mut inner = Vec::with_capacity(m*n);

        for _ in 0..m*n{
            inner.push(Default::default());
        }
        Self { m, n, inner }
    }

    fn from_iter(mut iter: impl Iterator<Item = T>, m:usize, n: usize) -> Self{
        let mut inner = Vec::with_capacity(m*n);
        for _ in 0..m*n{
            inner.push(iter.next().unwrap());
        }
        Self { m, n, inner }
    }

    fn column(&self, j:usize) -> Vec<&T>{
        let mut column = Vec::new();
        for i in 0..self.m{
            column.push(&self[(i, j)])
        }
        column
    }

    fn coin(&self) -> Self{
        let c = Self::new(self.m, self.n);
        for j in 0..self.n{
            for c in self.column(j){
                
            }

        }
        c
    }

}

impl<T:Display> Display for Matrix<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.m{
            for j in 0..self.n{
                write!(f, "{}", self[(i,j)])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<T> Index<(usize, usize)> for Matrix<T>{
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (i, j) = index;
        &self.inner[i*self.n + j]
    }
}

#[cfg(test)]
mod tests{
    use super::Matrix;

    #[test]
    fn init(){
        let words = [
            "favored",
            "thirsty",
            "whoever",
            "ghengis",
            "mounted",
            "freedom"
        ];
        let words = words.map(|s|s.to_string());
        let chars = words.iter().map(|s|s.chars()).collect::<Vec<_>>();
        let chars = chars.into_iter().flatten().collect::<Vec<_>>();
        let m = Matrix::from_iter(chars.into_iter(), 6,7);
        let c:Matrix<usize> = Matrix::new(6,7);
        // for c in m.column(0){

        //     println!("{}", c);
        // }
        println!("{m}");
        println!("{c}");

    }
}

