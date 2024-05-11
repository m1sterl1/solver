use terminal::{Result, words::{self, Words}};

use std::{env::args, io::{stdin, Stdin}};

fn print_words(words:&[String]){
    for (i, w) in words.iter().enumerate(){
        println!("{i:2}. {w}");
    }
}

fn read_index(stdin: &Stdin, max_index:usize) -> usize{
    loop{
        let mut buf = String::new();
        if let Ok(s) = stdin.read_line(&mut buf){
            if let Ok(i) = buf.trim().parse::<usize>(){
                if i < max_index{
                    return i
                } else {
                    println!("Index out of range, try again")
                }
            } else {
                println!("Error parsing index, try again")
            }
        } else {
            println!("Someghing wrong, try again")
        }
    }
}

fn run(path: &str) -> Result<()>{
    let stdin = stdin();
    let mut words = Words::from_file(path)?;
    print_words(words.words());
    let metrics = words.solve();
    println!("Please choose one of the word:");
    for (index, (word_index, m)) in metrics.iter().enumerate(){
        println!("\t{index}. {} \t{}", words.word(*word_index).unwrap(), m)
    }
    let i = read_index(&stdin, metrics.iter().count());

    println!("What number of intersections?");
    let (_,m) = &metrics[i];
    let intersections = m.groups().groups();
    for number in intersections.keys(){
        println!("{number}")
    }

    Ok(())


    

}

fn main() {
    let path = args().nth(1).expect("Usage terminal <PATH>");
    if let Err(e) = run(&path){
        println!("{e}");
    }
    
}
