use std::{ fs};

use clap::Parser;
use markov::Chain;

/// Builds a Markov chain based on an input file, and generates an arbitrary
/// amount of text based on the probabilities found.
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// The filename containing the words to build the Markov chain from.
    #[arg(short, long, value_name = "FILE")]
    file: String,

    /// The amount of words to generate using the input from `--file`.
    /// If omitted, will output sequences until the command is aborted.
    #[arg(short, long)]
    amount: Option<u64>,

    /// Emit a linebreak after this amount of characters. If not specified,
    /// will disregard linebreaks.
    #[arg(short, long)]
    lf: Option<usize>,
}

fn run(text: &str, opts: &Cli) {
    let split = text.split_ascii_whitespace();
    let vec: Vec<String> = split
        .map(|s| s.trim_matches(|c: char| c == '"' || c == '\''))
        .map(|w| w.to_string())
        // .map(|w| w.to_ascii_lowercase())
        .collect();

    let mut mat = Chain::new();

    let mut iterator = vec.iter();
    // Skip the first iteration or else we don't have a 'prev_word' yet, when
    // we iterate the regular way.
    let mut prev_word = iterator.next().unwrap();
    while let Some(curr_word) = iterator.next() {
        mat.insert(prev_word, curr_word);
        prev_word = curr_word;
    }

    let mut mat = mat.transition_matrix();

    let max = opts.amount.unwrap_or(u64::MAX);
    let mut cols = 0;
    for _ in 1..max {
        let next_word = mat.next().unwrap();
        print!("{next_word} ");

        if let Some(break_after) = opts.lf {
            cols += next_word.len();
            if cols >= break_after {
                println!();
                cols = 0;
            }
        }
    }
    println!();
}

//
fn main() {
    env_logger::init();

    let cli = Cli::parse();
    let s = fs::read_to_string(&cli.file).unwrap();
    run(&s, &cli);
}
