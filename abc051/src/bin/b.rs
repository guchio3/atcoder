#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        k: usize, s: usize
    }
    let mut res = 0;
    for i in 0..=min(k, s) {
        for j in 0..=min(k, s - i) {
            if s - i - j <= k {
                res += 1;
            }
        }
    }
    println!("{}", res)
}
