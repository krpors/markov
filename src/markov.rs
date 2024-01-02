use std::{collections::HashMap, fs, io::Error, ops::Deref, os::unix::thread};

use log::debug;
use rand::{
    distributions::{Distribution, WeightedIndex},
    rngs::ThreadRng,
    seq::{self, IteratorRandom, SliceRandom},
    thread_rng,
};

pub struct TransitionMatrix<'a> {
    rng: ThreadRng,
    current_word: &'a String,
    matrix: HashMap<&'a String, Vec<(&'a String, i64)>>,
}

impl<'a> TransitionMatrix<'a> {
    fn init_word(&mut self) {
        self.current_word = self
            .matrix
            .keys()
            .choose(&mut self.rng)
            .expect("Could not choose an initial word!")
    }

    pub fn new(map: &HashMap<String, HashMap<String, i64>>) -> TransitionMatrix {
        let mut mat = TransitionMatrix {
            rng: thread_rng(),
            // eh?
            current_word: map.iter().next().unwrap().0,
            matrix: HashMap::new(),
        };

        for (key, valuemap) in map {
            let mut vec: Vec<(&String, i64)> = Vec::new();
            valuemap.iter().for_each(|(k, v)| {
                vec.push((k, *v));
            });
            debug!("{key} has {:?} entries", vec);
            mat.matrix.insert(key, vec);
        }

        mat.init_word();

        debug!("Current word is {}", mat.current_word);

        mat
    }
}

impl <'a> Iterator for TransitionMatrix<'a> {
    type Item = &'a String;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.matrix.get(&self.current_word) {
            // debug!("===> {:?}", possibilities);
            let (next_word, _) = v.choose_weighted(&mut self.rng, |item| item.1).unwrap();
            self.current_word = next_word;
            Some(next_word)
        } else {
            self.init_word();
            Some(self.current_word)
        }
    }
}

pub struct Collector {
    occurence_map: HashMap<String, HashMap<String, i64>>,
}

// https://docs.rs/rand/0.7.2/rand/distributions/weighted/struct.WeightedIndex.html

impl Collector {
    pub fn new() -> Collector {
        Collector {
            occurence_map: HashMap::new()
        }
    }

    fn increment(map: &mut HashMap<String, i64>, key: &String) {
        map.entry(key.to_string())
            .and_modify(|v| {
                *v += 1;
            })
            .or_insert_with(|| 1);
    }

    pub fn insert(&mut self, curr_word: &String, next_possible_word: &String) {
        match self.occurence_map.get_mut(curr_word) {
            Some(valuemap) => {
                debug!("Incrementing {curr_word} -> {next_possible_word}");
                Self::increment(valuemap, next_possible_word);
            }
            None => {
                let mut newmap = HashMap::new();
                Self::increment(&mut newmap, next_possible_word);
                debug!("Adding {curr_word} -> {next_possible_word}");
                self.occurence_map.insert(curr_word.to_string(), newmap);
            }
        }
    }

    pub fn transition_matrix(&mut self) -> TransitionMatrix {
        TransitionMatrix::new(&self.occurence_map)
    }

    // rand::seq::SliceRandom choose_weighted on tuple
    // https://stackoverflow.com/questions/71092791/how-to-select-a-random-key-from-an-unorderedmap-in-near-rust-sdk

    pub fn print(&self) {
        println!("The chain has {} entries", self.occurence_map.len());
        for (key, valuemap) in &self.occurence_map {
            print!("{key} -> ");
            for (key, value) in valuemap {
                print!("({key}:{value}), ");
            }
            println!();
        }
    }
}
