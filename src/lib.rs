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

impl<'a, K: Hash + Eq + Display> Iterator for TransitionMatrix<'a, K> {
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

/// This struct contains functionality to build a discrete-time Markov chain,
/// using any type that implements the traits [Hash], [Eq] and [Display].
/// A Markov chain can be used to build a sequence of things using weighted randomness.
///
/// Example applications:
/// - Generate sequences of words
/// - Create music
/// - Find a next state for an NPC in a game
/// - ... and more.
///
/// The generic parameter `K` denotes the actual type to be put in the chain.
/// For instance, it could be a `struct`, `String`, or any other type, as long
/// as it implements the required traits. The Markov-chain itself is backed by
/// a [HashMap].
///
/// For more thorough information on what a Markov chain is, consult the [Wikipedia page
/// on Markov chains](https://en.wikipedia.org/wiki/Markov_chain).
///
/// Examples:
///
/// ```
/// use markov::Chain;
///
/// let mut chain = Chain::new();
/// chain.insert("I", "see");
/// chain.insert("you", "there");
/// chain.insert("I", "know");
/// chain.insert("you", "everytime");
///
/// let mut transmat = chain.transition_matrix();
///
/// for _ in 1..10 {
///     print!("{} ", transmat.next().unwrap());
/// }
///
/// // Possible output:
///
/// // everytime I see you everytime I know I see
///
/// ```
pub struct Chain<K> {
    chain: HashMap<K, HashMap<K, i64>>,
}

impl<K: Hash + Eq + Display> Chain<K> {
    /// Creates a new Markov-chain using a specified key type.
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

    /// Insert two values in the chain, where `current` is the source state, and
    /// `next` is a state which can be followed after `current`.
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

    /// Generates a transition matrix from the initial chain. This matrix
    /// can be used to traverse the Markov-chain using the [Iterator] trait.
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
