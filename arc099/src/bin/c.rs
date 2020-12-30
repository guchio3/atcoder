#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        n: usize, k: usize,
        _a: [usize; n]
    }
    let res = (n - 1) / (k - 1) + ((n - 1) % (k - 1) != 0) as usize;
    println!("{}", res);
}
