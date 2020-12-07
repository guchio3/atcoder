#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize, k: f64
    }

    let mut res = 0.;
    for i in 1..=n {
        if i >= k as usize {
            res += 1.;
            continue;
        }
        // println!("{}", (k / i as f64).log2().ceil());
        res += (0.5 as f64).powi(((k / i as f64).log2().ceil()) as i32);
    }
    println!("{}", res / n as f64);
}
