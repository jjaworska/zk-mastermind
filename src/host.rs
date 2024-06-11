use rand::Rng;
use crate::utils::{same, common};
const CHARSET: &[u8] = b"abcdefgh";
const SEQUENCE_LEN: usize = 4;

pub trait Host {
    fn guess(&self, sequence: String) -> (usize, usize);
    fn new () -> Self;
}

pub struct HonestHost {
    sequence: String,
}

impl Host for HonestHost{
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
    fn guess(&self, sequence: String)-> (usize, usize) {
        return (same(self.sequence.clone(), sequence.clone()), common(self.sequence.clone(), sequence.clone()))
    }
}

pub struct EvilHost {}

impl Host for EvilHost{
    fn new () -> Self {
        EvilHost { }
    }

    fn guess(&self, _sequence: String)-> (usize, usize) {
        return (0, 0);
    }
}