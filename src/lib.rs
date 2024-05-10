use std::error::Error;

mod matrix;
mod words;
mod utils;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
