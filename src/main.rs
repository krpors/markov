use std::{cmp::Ordering, collections::HashMap, fs::File};

use log::debug;
use multimap::MultiMap;
use rand::{
    distributions::{Distribution, WeightedIndex},
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
                debug!("Incremented '{key}' to {}", v);
            })
            .or_insert_with(|| {
                debug!("Inserting default value (1) for '{key}'");
                1
            });
    }

    pub fn insert(&mut self, curr_word: &String, next_possible_word: &String) {
        match self.matrix.get_mut(curr_word) {
            Some(valuemap) => {
                Self::increment(valuemap, next_possible_word);
            }
            None => {
                let mut newmap = HashMap::new();
                Self::increment(&mut newmap, next_possible_word);
                self.matrix.insert(curr_word.to_string(), newmap);
            }
        }
    }

    pub fn next(&self) {
        let mut start = self.matrix.get("I").unwrap();
        print!("I");
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

fn analyse() {
    let text = "I live in a house near the mountains.
    I have two brothers and one sister, and I was born last.
    My father teaches mathematics, and my mother is a nurse at
    a big hospital. My brothers are very smart and work hard in
    school. My sister is a nervous girl, but she is very kind.
    My grandmother also lives with us. She came from Italy when
    I was two years old. She has grown old, but she is still very
    strong. She cooks the best food!";

    let split = text.split_ascii_whitespace();
    let vec: Vec<String> = split
        // .map(|s| s.trim_matches(|c: char| c.is_ascii_punctuation()))
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

    mat.next();

    //  0.25, 0.25, 0.50
}

fn main() {
    env_logger::init();

    analyse();
}
