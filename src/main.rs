use std::{collections::HashMap, fs};

use clap::Parser;
use markov::Collector;

mod markov;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {

    /// The filename containing the words to build the Markov chain from.
    #[arg(short, long, value_name = "FILE")]
    file: String,

    /// The amount of words to generate using the input from `--file`.
    #[arg(short, long)]
    amount: u64,
}

fn analyse(text: &str, amt: u64) {
    let split = text.split_ascii_whitespace();
    let vec: Vec<String> = split
        .map(|s| s.trim_matches(|c: char| c == '"' || c == '\''))
        .map(|w| w.to_string())
        // .map(|w| w.to_ascii_lowercase())
        .collect();

    let mut mat = Collector::new();

    let mut iterator = vec.iter();
    // Skip the first iteration or else we don't have a 'prev_word' yet, when
    // we iterate the regular way.
    let mut prev_word = iterator.next().unwrap();
    while let Some(curr_word) = iterator.next() {
        mat.insert(prev_word, curr_word);
        prev_word = curr_word;
    }

    // mat.print();

    let mut mat = mat.transition_matrix();
    let mut cols = 0;
    for _ in 1..amt {
        let next = mat.next().unwrap();
        print!("{next} ");
        cols += next.len();
        if cols >= 80 {
            println!();
            cols = 0;
        }
    }
    println!();
}

fn main() {
    env_logger::init();

    let cli = Cli::parse();

    let s = fs::read_to_string(cli.file).unwrap();
    analyse(&s, cli.amount);
}
