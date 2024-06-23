use itertools::Itertools;
use rand::Rng;
use std::collections::{HashMap, HashSet};
use crate::crypto::Code;
use crate::utils::{common, hash, same, string_to_code};
use crate::proof::{prove, prove_guess, Proof};
const CHARSET: &[u8] = b"abcdefgh";
const SEQUENCE_LEN: usize = 4;

pub trait Host {
    fn new () -> Self;
    fn get_hash_with_proof(&self) -> ([u8; 32], Proof);
    fn guess(&mut self, sequence: String) -> (usize, usize, Proof);
}

pub struct HonestHost {
    sequence: String,
    salt: [u8; 32],
    hash: [u8; 32],
}

impl Host for HonestHost {
    fn new () -> Self {
        let mut rng = rand::thread_rng();
        let random_seq : String = (0..SEQUENCE_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
        let code = string_to_code(random_seq.clone());
        let (hash, salt) = hash(code);
        HonestHost {
            sequence: random_seq,
            salt,
            hash,
        }
    }
    fn guess(&mut self, sequence: String)-> (usize, usize, Proof) {
        let (correct, common) = (same(self.sequence.clone(), sequence.clone()), common(self.sequence.clone(), sequence.clone()));
        let proof = prove_guess(string_to_code(self.sequence.clone()), string_to_code(sequence), self.salt, self.hash, correct as u8, (common-correct) as u8);
        return (correct, common, proof)
    }
    
    fn get_hash_with_proof(&self) -> ([u8; 32], Proof) {
        let code = string_to_code(self.sequence.clone());
        println!("{}", self.sequence);
        println!("{:?}", code);
        let proof = prove(code, self.salt, self.hash);
        (self.hash, proof)
    }
}

pub struct EvilHost {}

impl Host for EvilHost{ // host which always answers with (0, 0)
    fn new () -> Self {
        EvilHost { }
    }

    fn guess(&mut self, sequence: String)-> (usize, usize, Proof) {
        let code = Code{colors:[1, 1, 1, 1]};
        let (hash, salt) = hash(code.clone());
        let proof = prove_guess(code, string_to_code(sequence), salt, hash, 0, 0);
        return (0, 0, proof);
    }
    
    fn get_hash_with_proof(&self) -> ([u8; 32], Proof) {
        let code = Code{colors:[1, 1, 1, 1]};
        let (hash, salt) = hash(code.clone());
        let proof = prove(code, salt, hash);
        (hash, proof)
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

    fn guess(&mut self, sequence: String)-> (usize, usize, Proof) {
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
        let seq = self.possible_sequences.iter().next().unwrap().clone();
        let code = string_to_code(seq);
        let (hash, salt) = hash(code.clone());
        let proof = prove_guess(code, string_to_code(sequence), salt, hash, ans.0 as u8, ans.1 as u8);
        return (ans.0, ans.1, proof);
    }
    
    fn get_hash_with_proof(&self) -> ([u8; 32], Proof) {
        let seq = self.possible_sequences.iter().next().unwrap().clone();
        let code = string_to_code(seq);
        let (hash, salt) = hash(code.clone());
        let proof = prove(code, salt, hash);
        (hash, proof)
    }
}