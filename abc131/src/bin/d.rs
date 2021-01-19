#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        n: usize,
        mut ab: [(usize, usize); n]
    }
    ab.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap());
    ab.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());
    let mut current_time = 0;
    for (a_i, b_i) in ab {
        if current_time + a_i > b_i {
            println!("No");
            return;
        } else {
            current_time += a_i;
        }
    }
    println!("Yes");
}
