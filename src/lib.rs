use std::error::Error;

mod matrix;
mod utils;
pub mod words;
pub mod metric;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
