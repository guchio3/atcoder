#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut exists = HashSet::new();
    for a_i in a {
        if exists.contains(&a_i) {
            println!("NO");
            return;
        }
        exists.insert(a_i);
    }
    println!("YES");
}
