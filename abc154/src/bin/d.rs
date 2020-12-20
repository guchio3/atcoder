#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize, k: usize,
        p: [usize; n]
    }
    let mut k_sum = 0;
    for i in 0..k {
        k_sum += p[i];
    }
    let mut best_i = 0;
    let mut best_k_sum = k_sum;
    for i in 1..=(n - k) {
        k_sum += p[i + k - 1];
        k_sum -= p[i - 1];
        if k_sum > best_k_sum {
            best_k_sum = k_sum;
            best_i = i;
        }
    }
    let mut res = 0.;
    for i in best_i..best_i + k {
        res += (1 + p[i]) as f64 / 2.;
    }

    println!("{}", res);
}
