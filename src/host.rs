use itertools::Itertools;
use rand::Rng;
use std::collections::{HashMap, HashSet};
use crate::utils::{same, common};
const CHARSET: &[u8] = b"abcdefgh";
const SEQUENCE_LEN: usize = 4;

pub trait Host {
    fn new () -> Self;
    fn guess(&mut self, sequence: String) -> (usize, usize);
}

pub struct HonestHost {
    sequence: String,
}

impl Host for HonestHost {
    fn new () -> Self {
        let mut rng = rand::thread_rng();
        let random_seq = (0..SEQUENCE_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
        HonestHost {
            sequence: random_seq,
        }
    }
    fn guess(&mut self, sequence: String)-> (usize, usize) {
        return (same(self.sequence.clone(), sequence.clone()), common(self.sequence.clone(), sequence.clone()))
    }
}

pub struct EvilHost {}

impl Host for EvilHost{ // host which always answers with (0, 0)
    fn new () -> Self {
        EvilHost { }
    }

    fn guess(&mut self, _sequence: String)-> (usize, usize) {
        return (0, 0);
    }
}

pub struct CheatingHost { //host which always gives worst case answer for player
    possible_sequences: HashSet<String>,
    worst_case_sequences: HashMap<(usize, usize), i32>
}

impl Host for CheatingHost{
    fn new () -> Self {
        let characters = vec!["a", "b", "c", "d", "e", "f", "g", "h"];
        let pairs : Vec<_> = characters.iter()
        .cartesian_product(characters.iter())
        .map(|(&a, &b)| a.to_owned() + b)
        .collect();
        let quadruples : Vec<_> = pairs.iter()
        .cartesian_product(pairs.iter())
        .map(|(a, b)| a.to_owned() + b)
        .collect();
        let s: HashSet<String, _> = HashSet::from_iter(quadruples);
        let mut m: HashMap<(usize, usize), i32> = HashMap::new();
        for i in 0usize..=4 {
            for j in i..=4 {
                m.insert((i, j), 0);
            }
        }
        CheatingHost {
            possible_sequences: s,
            worst_case_sequences: m,
        }
    }

    fn guess(&mut self, sequence: String)-> (usize, usize) {
        let mut m = self.worst_case_sequences.clone();
        for seq in self.possible_sequences.clone() {
            let (same, common) = (same(seq.clone(), sequence.clone()), common(seq.clone(), sequence.clone()));
            let old = m.get(&(same, common)).unwrap();
            m.insert((same, common), old+1);
        }
        let ans = m.iter()
            .max_by(|a, b| (a.1, - (a.0.0 as i32), - (a.0.1 as i32)).cmp(&(b.1, - (b.0.0 as i32), - (b.0.1 as i32))))
            .unwrap().0;
        for seq in self.possible_sequences.clone() {
            let (same, common) = (same(seq.clone(), sequence.clone()), common(seq.clone(), sequence.clone()));
            if (same, common) != *ans {
                self.possible_sequences.remove(&seq);
            }
        }
        return *ans;
    }
}