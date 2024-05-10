use std::error::Error;

mod matrix;
mod utils;
pub mod words;
mod groups;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
