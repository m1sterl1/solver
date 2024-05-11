use std::env::args;
use terminal::words::Words;
use terminal::Result;

fn run(path: &str) -> Result<()> {
    let words = Words::from_file(path)?;
    words.build_tree().run();
    Ok(())
}

fn main() {
    let path = args().nth(1).expect("Usage terminal <PATH>");
    if let Err(e) = run(&path) {
        println!("{e}");
    }
}
