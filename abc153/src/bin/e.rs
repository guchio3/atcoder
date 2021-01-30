#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        mut h: usize, n: usize,
        mut ab: [(usize, usize); n]
    }
    let mut dp = vec![1_000_000_000; h + 1];
    for &(a_i, b_i) in ab.iter() {
        let index = min(a_i, h);
        dp[index] = min(b_i, dp[index]);
    }
    for i in 1..=h {
        if dp[i] == 1_000_000_000 {
            continue;
        }
        for &(a_i, b_i) in ab.iter() {
            let index = min(i + a_i, h);
            dp[index] = min(dp[i] + b_i, dp[index]);
        }
    }
    println!("{}", dp[h]);
}

// fn main() {
//     input! {
//         mut h: i64, n: usize,
//         mut ab: [(i64, i64); n]
//     }
//     // ab.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());
//     let mut res = 0;
//     while h > 0 {
//         let mut best_i = 0;
//         let mut best_eff = 0.;
//         for i in 0..ab.len() {
//             let (mut a_i, b_i) = ab[i];
//             if a_i > h {
//                 a_i = h;
//             }
//             let eff = a_i as f64 / b_i as f64;
//             if eff >= best_eff {
//                 best_i = i;
//                 best_eff = eff;
//             }
//         }
//         h -= ab[best_i].0;
//         res += ab[best_i].1;
//         // println!(
//         //     "h: {}, best_i: {}, res: {}, ab[best_i]: {:?}",
//         //     h, best_i, res, ab[best_i]
//         // );
//     }
//     println!("{}", res);
// }
