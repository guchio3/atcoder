#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        s: Chars
    }
    let res = vec!['x'; s.len()];
    println!("{}", res.into_iter().join(""));
}
