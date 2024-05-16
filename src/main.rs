use std::env::args;
use std::fs::read_to_string;
use serde::Deserialize;
use terminal::words::Words;
use terminal::Result;


#[derive(Deserialize)]
struct JsonResponse{
    words: Vec<String>,
}

fn run() -> Result<()> {
    let path = args().nth(1).ok_or("Usage terminal <PATH>")?;
    let content = read_to_string(path)?;
    let response:JsonResponse = serde_json::from_str(&content)?;
    let words = Words::from_iter(response.words.iter())?;
    words.build_tree().run();
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("{e}");
    }
}
