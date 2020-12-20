#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

// fn main() {
//     input! {
//         h: usize, w: usize,
//         a: [[usize; w]; h]
//     }
//     let sum = a.iter().fold(0, |sum, a_i| {
//         sum + a_i.iter().fold(0, |sum, a_ij| sum + a_ij)
//     });
//     let min_value = a.iter().fold(101, |min_value, a_i| {
//         min(min_value, a_i.iter().fold(101, |min_value, a_ij| min(min_value, *a_ij)))
//     });
//     println!("{}", sum - min_value * h * w);
// }

fn main() {
    input! {
        h: usize, w: usize,
        a: [[usize; w]; h]
    }
    let sum: usize = a.iter().map(|a_i| a_i.iter().sum::<usize>()).sum();
    let min_value = a.iter().map(|a_i| a_i.iter().min().unwrap()).min().unwrap();
    println!("{}", sum - min_value * h * w);
}
