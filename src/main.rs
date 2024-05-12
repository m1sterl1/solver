use std::env::args;
use terminal::words::Words;
use terminal::Result;

fn run() -> Result<()> {
    let path = args().nth(1).ok_or("Usage terminal <PATH>")?;
    let words = Words::from_file(path)?;
    words.build_tree().run();
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("{e}");
    }
}
