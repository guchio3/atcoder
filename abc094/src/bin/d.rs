#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        n: usize, 
        a: [usize; n]
    }
    let &max_a_i = a.iter().max().unwrap();
    
    let mut best_diff = 0;
    let mut res = max_a_i;
    for a_i in a {
        let mut diff = max_a_i - a_i;
        if diff > (max_a_i / 2) {
            diff = a_i;
        }
        if diff >= best_diff {
            best_diff = diff;
            res = a_i;
        }
    }

    println!("{} {}", max_a_i, res);
}
