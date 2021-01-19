#![allow(unused_imports)]
use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

#[fastout]
fn main() {
    input! {
        n: usize,
        x: [usize; n]
    }
    let mut sorted_x: Vec<usize> = x.iter().map(|a| *a).collect();
    sorted_x.sort();
    let candidate1 = sorted_x[(n - 1) / 2];
    let candidate2 = sorted_x[n / 2];
    for i in 0..n {
        let x_i = x[i];
        if x_i <= candidate1 {
            println!("{}", candidate2);
        } else {
            println!("{}", candidate1);
        }
    }
}
