use std::error::Error;

mod matrix;
pub mod metric;
mod utils;
pub mod words;
mod tree;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
