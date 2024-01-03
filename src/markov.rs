use std::hash::Hash;
use std::{collections::HashMap, fmt::Display};

use rand::rngs::ThreadRng;
use rand::seq::{IteratorRandom, SliceRandom};
use rand::thread_rng;

pub struct TransitionMatrix<'a, K> {
    rng: ThreadRng,
    current: &'a K,
    matrix: HashMap<&'a K, Vec<(&'a K, i64)>>,
}

impl<'a, K: Hash + Eq + Display> TransitionMatrix<'a, K> {
    fn init_word(&mut self) {
        self.current = self
            .matrix
            .keys()
            .choose(&mut self.rng)
            .expect("Could not choose an initial word!")
    }

    fn new(map: &HashMap<K, HashMap<K, i64>>) -> TransitionMatrix<K> {
        let mut mat = TransitionMatrix {
            rng: thread_rng(),
            // TODO: better way to get the current word?
            current: map.iter().next().unwrap().0,
            matrix: HashMap::new(),
        };

        for (key, valuemap) in map {
            let mut vec: Vec<(&K, i64)> = Vec::new();
            valuemap.iter().for_each(|(k, v)| {
                vec.push((k, *v));
            });
            // debug!("{key} has {:?} entries", vec);
            mat.matrix.insert(key, vec);
        }

        mat.init_word();

        mat
    }
}


impl <'a, K: Hash + Eq + Display> Iterator for TransitionMatrix<'a, K> {
    type Item = &'a K;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.matrix.get(&self.current) {
            let (next_word, _) = v.choose_weighted(&mut self.rng, |item| item.1).unwrap();
            self.current = next_word;
            Some(next_word)
        } else {
            self.init_word();
            Some(self.current)
        }
    }
}

pub struct Chain<K> {
    chain: HashMap<K, HashMap<K, i64>>,
}

impl<K: Hash + Eq + Display> Chain<K> {
    pub fn new() -> Chain<K> {
        Chain {
            chain: HashMap::new(),
        }
    }

    fn increment(map: &mut HashMap<K, i64>, next: K) {
        map.entry(next)
            .and_modify(|v| {
                *v += 1;
            })
            .or_insert_with(|| 1);
    }

    pub fn insert(&mut self, current: K, next: K) {
        match self.chain.get_mut(&current) {
            Some(valuemap) => {
                Self::increment(valuemap, next);
            }
            None => {
                let mut newmap = HashMap::new();
                Self::increment(&mut newmap, next);
                self.chain.insert(current, newmap);
            }
        }
    }

    pub fn transition_matrix(&mut self) -> TransitionMatrix<K> {
        TransitionMatrix::new(&self.chain)
    }

    pub fn print(&self) {
        println!("The chain has {} entries", self.chain.len());
        for (key, valuemap) in &self.chain {
            print!("{key} -> ");
            for (key, value) in valuemap {
                print!("({key}:{value}), ");
            }
            println!();
        }
    }
}
