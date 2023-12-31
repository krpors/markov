use std::{collections::HashMap, fs, ops::Deref, os::unix::thread};

use log::debug;
use rand::{
    distributions::{Distribution, WeightedIndex},
    seq::{self, SliceRandom, IteratorRandom},
    thread_rng,
};

struct StateMatrix {
    matrix: HashMap<String, HashMap<String, i64>>,
}

// https://docs.rs/rand/0.7.2/rand/distributions/weighted/struct.WeightedIndex.html

impl StateMatrix {
    fn increment(map: &mut HashMap<String, i64>, key: &String) {
        map.entry(key.to_string())
            .and_modify(|v| {
                *v += 1;
            })
            .or_insert_with(|| 1);
    }

    pub fn insert(&mut self, curr_word: &String, next_possible_word: &String) {
        match self.matrix.get_mut(curr_word) {
            Some(valuemap) => {
                debug!("Incrementing {curr_word} -> {next_possible_word}");
                Self::increment(valuemap, next_possible_word);
            }
            None => {
                let mut newmap = HashMap::new();
                Self::increment(&mut newmap, next_possible_word);
                debug!("Adding {curr_word} -> {next_possible_word}");
                self.matrix.insert(curr_word.to_string(), newmap);
            }
        }
    }

    pub fn calc(&self) {
        let mut mat: HashMap<String, Vec<(String, i64)>> = HashMap::new();
        for (key, valuemap) in &self.matrix {
            let mut vec: Vec<(String, i64)> = Vec::new();
            valuemap.iter().for_each(|(k, v)| {
                vec.push((k.to_string(), *v));
            });
            debug!("{key} has {:?} entries", vec);
            mat.insert(key.to_string(), vec);
        }

        let mut rng = thread_rng();
        let mut word = mat.keys().choose(&mut rng).unwrap();
        print!("{word} ");
        for _ in 1..200 {
            let balls = mat.get(word).unwrap();
            let (w, _) = balls.choose_weighted(&mut rng, |item| item.1).unwrap();
            print!("{w} ");
            word = w;
        }
    }

    // rand::seq::SliceRandom choose_weighted on tuple
    // https://stackoverflow.com/questions/71092791/how-to-select-a-random-key-from-an-unorderedmap-in-near-rust-sdk

    pub fn next(&self) {
        use rand::seq::SliceRandom;

        let keys: Vec<&String> = self.matrix.keys().collect();
        let random_key = keys.choose(&mut thread_rng()).unwrap();

        let mut start = self.matrix.get(*random_key).unwrap();

        print!("{random_key}");

        for _ in 1..200 {
            // TODO optimize this
            let mut choices: Vec<&str> = vec![];
            let mut weights: Vec<i64> = vec![];
            for (k, v) in start {
                choices.push(k);
                weights.push(*v);
            }
            let dist = WeightedIndex::new(&weights).unwrap();
            let mut rng = thread_rng();

            let sample = dist.sample(&mut rng);
            let nextword = choices[sample];

            print!(" {nextword}");

            start = if let Some(x) = self.matrix.get(nextword) {
                x
            } else {
                break;
            }
        }
        println!();
    }

    pub fn print(&self) {
        for (key, valuemap) in &self.matrix {
            println!("{key}");
            for (key, value) in valuemap {
                println!(" - {key} = {value}");
            }
        }
    }
}

fn analyse(text: &str) {
    let split = text.split_ascii_whitespace();
    let vec: Vec<String> = split
        .map(|s| s.trim_matches(|c: char| c == '"' || c == '\''))
        .map(|w| w.to_string())
        // .map(|w| w.to_ascii_lowercase())
        .collect();

    let mut mat = StateMatrix {
        matrix: HashMap::new(),
    };

    let mut iterator = vec.iter();
    // Skip the first iteration or else we don't have a 'prev_word' yet, when
    // we iterate the regular way.
    let mut prev_word = iterator.next().unwrap();
    while let Some(curr_word) = iterator.next() {
        mat.insert(prev_word, curr_word);
        prev_word = curr_word;
    }

    // mat.print();

    mat.calc();

    // mat.next();

    //  0.25, 0.25, 0.50
}

fn main() {
    env_logger::init();

    let s = fs::read_to_string("./input1.txt").unwrap();
    analyse(&s);
}
