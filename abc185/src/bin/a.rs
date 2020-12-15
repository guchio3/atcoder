#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        a1: usize, a2: usize, a3: usize, a4:usize
    }
    println!("{}", min(a1, min(a2, min(a3, a4))));
}
