use std::cmp::max;
use itertools::Itertools;
use ark_std::iterable::Iterable;
use sha2::{Digest, Sha256};
use crate::crypto::{Code, CODE_LENGTH};

pub fn same(sa: String, sb: String) -> usize {
    let a: Vec<_> = sa.chars().collect();
    let b: Vec<_> = sb.chars().collect();
    let mut ans = 0;
    for i in 0..4 {
        ans += (a[i] == b[i]) as usize;
    }
    ans
}

pub fn common(sa: String, sb: String) -> usize {
    let a: Vec<_> = sa.chars().sorted().collect();
    let b: Vec<_> = sb.chars().sorted().collect();
    let mut array: [[usize; 4]; 4] = [[0; 4]; 4];
    if a[0] == b[0] {
        array[0][0] = 1
    }
    for j in 1..4 {
        array[0][j] = array[0][j-1];
        if a[0] == b[j] {
            array[0][j] = 1;
        }
    }
    for i in 1..4 {
        array[i][0] = array[i-1][0];
        if a[i] == b[0] {
            array[i][0] = 1;
        }
    }
    for i in 1..4 {
        for j in 1..4 {
            array[i][j] = max(
                max(array[i-1][j], array[i][j-1]),
                array[i-1][j-1] + ((a[i] == b[j]) as usize)
            )
        }
    }
    array[3][3]
}

pub fn string_to_code(seq: String) -> Code {
    let s: Vec<_> = seq.chars().collect();
    let mut colors:[u8; CODE_LENGTH] =  [0, 0, 0, 0];
    for i in 0..4 {
        colors[i] = s[i] as u8 - 'a' as u8;
    }
    Code{colors}
}

pub fn hash (code: Code) -> ([u8; 32], [u8; 32]){
    let salt = rand::random::<[u8; 32]>();

    let mut hasher = Sha256::new();

    code
        .colors
        .iter()
        .for_each(|col| hasher.update([col]));
    hasher.update(salt);

    let hash_result = hasher.finalize();

    (hash_result.into(), salt)
}