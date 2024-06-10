use std::cmp::max;
use itertools::Itertools;

pub fn same(sa: String, sb: String) -> i32 {
    let a: Vec<_> = sa.chars().collect();
    let b: Vec<_> = sb.chars().collect();
    let mut ans = 0;
    for i in 0..4 {
        ans += (a[i] == b[i]) as i32;
    }
    ans
}

pub fn common(sa: String, sb: String) -> i32 {
    let a: Vec<_> = sa.chars().sorted().collect();
    let b: Vec<_> = sb.chars().sorted().collect();
    let mut array = [[0; 4]; 4];
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
                array[i-1][j-1] + ((a[i] == b[j]) as i32)
            )
        }
    }
    array[3][3]
}
