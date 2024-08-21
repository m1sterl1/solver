use std::error::Error;

mod matrix;
pub mod solver;
pub(crate) mod tree;
mod utils;
mod words_solver;

pub use solver::Solver;
pub use tree::{Answer, Guess};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
