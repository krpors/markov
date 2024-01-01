use std::{collections::HashMap, fs, io::Error, ops::Deref, os::unix::thread};

use log::debug;
use rand::{
    distributions::{Distribution, WeightedIndex},
    rngs::ThreadRng,
    seq::{self, IteratorRandom, SliceRandom},
    thread_rng,
};

struct TransitionMatrix {
    rng: ThreadRng,
    current_word: String,
    matrix: HashMap<String, Vec<(String, i64)>>,
}

impl TransitionMatrix {
    pub fn new(map: &HashMap<String, HashMap<String, i64>>) -> TransitionMatrix {
        let mut mat = TransitionMatrix {
            rng: thread_rng(),
            current_word: String::new(),
            matrix: HashMap::new(),
        };

        for (key, valuemap) in map {
            let mut vec: Vec<(String, i64)> = Vec::new();
            valuemap.iter().for_each(|(k, v)| {
                vec.push((k.to_string(), *v));
            });
            debug!("{key} has {:?} entries", vec);
            mat.matrix.insert(key.to_string(), vec);
        }

        mat.current_word = mat
            .matrix
            .keys()
            .choose(&mut mat.rng)
            .expect("Could not choose an initial word!")
            .to_string();

        debug!("Current word is {}", mat.current_word);

        mat
    }
}

impl Iterator for TransitionMatrix {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let possibilities = self.matrix.get(&self.current_word).unwrap();
        // debug!("===> {:?}", possibilities);
        let (next_word, _) = possibilities
            .choose_weighted(&mut self.rng, |item| item.1)
            .unwrap();
        self.current_word = next_word.to_string();
        Some(next_word.to_string())
    }
}

struct Collector {
    matrix: HashMap<String, HashMap<String, i64>>,
}

// https://docs.rs/rand/0.7.2/rand/distributions/weighted/struct.WeightedIndex.html

impl Collector {
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

    pub fn transition_matrix(&mut self) -> TransitionMatrix {
        TransitionMatrix::new(&self.matrix)
    }

    // rand::seq::SliceRandom choose_weighted on tuple
    // https://stackoverflow.com/questions/71092791/how-to-select-a-random-key-from-an-unorderedmap-in-near-rust-sdk

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

    let mut mat = Collector {
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

    let mut mat = mat.transition_matrix();
    for _ in  1..200 {
        print!("{} ", mat.next().unwrap());
    }

    // mat.next();

    //  0.25, 0.25, 0.50
}

fn map_to_vec<'a>(key: &String, map: &'a HashMap<String, String>) -> Vec<&'a String>{
    let v = map.get(key);
    vec![v.unwrap()]
}

fn main() {
    env_logger::init();

    let mut mymap = HashMap::new();
    mymap.insert(String::from("Kevin"), String::from("Pors"));
    let v = map_to_vec(&String::from("Kevin"), &mymap);
    println!("{:?}", v);

    // let s = fs::read_to_string("./input1.txt").unwrap();
    // analyse(&s);
}
