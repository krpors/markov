use std::{collections::HashMap, fs};

use markov::Collector;

mod markov;


fn analyse(text: &str) {
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
    for _ in 1..2000 {
        print!("{} ", mat.next().unwrap());
    }

}

fn main() {
    env_logger::init();

    let s = fs::read_to_string("./input1.txt").unwrap();
    analyse(&s);
}
