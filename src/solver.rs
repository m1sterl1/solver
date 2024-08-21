use crate::tree::{Answer, Guess, Tree};
use crate::words_solver::WordsSolver;
use crate::Result;

pub struct Solver {
    tree: Tree,
}

impl Solver {
    pub fn new(words: Vec<String>, guesses: Vec<Guess>) -> Result<Self> {
        let words_solver = WordsSolver::from_iter(words.iter())?;
        let mut tree = words_solver.build_tree();
        for guess in guesses {
            tree.next_answer(&guess);
        }
        Ok(Self { tree })
    }

    pub fn answer(&self) -> Answer {
        self.tree.answer()
    }

    pub fn next_answer(&mut self, guess: &Guess) -> Answer {
        self.tree.next_answer(guess)
    }
}
