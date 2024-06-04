use std::error::Error;

mod matrix;
mod tree;
mod utils;
mod words_solver;
pub mod solver;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
