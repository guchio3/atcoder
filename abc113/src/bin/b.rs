#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        n: usize,
        t: f64, a: f64,
        h: [f64; n]
    }
    let mut temp_diffs = vec![];
    for i in 0..n {
        let hi = h[i];
        let temp_diff = (a - t + hi * 0.006).abs();
        temp_diffs.push((temp_diff, i + 1));
    }
    temp_diffs.sort_by(|x, y| x.partial_cmp(&y).unwrap());
    println! {"{}", temp_diffs[0].1};
}

// fn main() {
//     input! {
//         n: usize,
//         t: i64, a: i64,
//         h: [i64; n]
//     }
//     let mut res = 0;
//     let mut min_temp_diff = 1_000_000_000_000;
//     for i in 0..n {
//         let h_i = h[i];
//         let temp_diff = (a * 1000 - t * 1000 + h_i * 6).abs();
//         if temp_diff < min_temp_diff {
//             min_temp_diff = temp_diff;
//             res = i + 1;
//         }
//     }
//     println!("{}", res);
// }
