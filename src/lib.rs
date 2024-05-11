use std::error::Error;

mod matrix;
pub mod tree;
mod utils;
pub mod words;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
