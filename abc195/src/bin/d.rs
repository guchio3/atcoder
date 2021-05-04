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
        n: usize, m: usize, q: usize,
        mut wv: [(usize, usize); n],
        x: [usize; m],
        query: [(usize, usize); q]
    }

    wv.sort_by(|x, y| y.1.partial_cmp(&x.1).unwrap());

    for (li, ri) in query {
        let mut hako_indexes = vec![100; m];
        for i in 0..n {
            let mut best_j = 0;
            let mut best_j_diff = 1_000_000_000;
            for j in 0..m {
                if (li - 1 > j || j > ri - 1) && hako_indexes[j] == 100 && x[j] >= wv[i].0 {
                    let j_diff = x[j] - wv[i].0;
                    if best_j_diff > j_diff {
                        best_j = j;
                        best_j_diff = j_diff;
                    }
                }
            }
            if best_j_diff != 1_000_000_000 {
                hako_indexes[best_j] = i;
            }
        }

        let mut res = 0;
        for j in 0..m {
            if hako_indexes[j] < 100 {
                res += wv[hako_indexes[j]].1;
            }
        }
        println!("{}", res);
    }
}

// fn main() {
//     input! {
//         n: usize, m: usize, q: usize,
//         mut wv: [(usize, usize); n],
//         x: [usize; m],
//         query: [(usize, usize); q]
//     }
//
//     let mut hako_indexes = vec![100; m];
//     wv.sort_by(|x, y| y.1.partial_cmp(&x.1).unwrap());
//     for i in 0..n {
//         let mut best_j = 0;
//         let mut best_j_diff = 1_000_000_000;
//         for j in 0..m {
//             if hako_indexes[j] == 100 && x[j] >= wv[i].0 {
//                 let j_diff = x[j] - wv[i].0;
//                 if best_j_diff > j_diff {
//                     best_j = j;
//                     best_j_diff = j_diff;
//                 }
//             }
//         }
//         if best_j_diff != 1_000_000_000 {
//             hako_indexes[best_j] = i;
//         }
//     }
//
//     for (li, ri) in query {
//         let mut switch_wvs = vec![];
//         for i in li..=ri {
//             switch_wvs.push(wv[hako_indexes[i]]);
//         }
//         switch_wvs.sort_by(|x, y| y.1.partial_cmp(&x.1).unwrap());
//         for i in 0..switch_wvs.len() {
//             let mut best_j = 0;
//             let mut best_j_diff = 1_000_000_000;
//             for j in 0..m {
//                 if (li > j || j > ri) &&  && wv[hako_indexes[j]].1 >= switch_wvs[i].1 {
//                     let j_diff = x[j] - wv[i].0;
//                     if best_j_diff > j_diff {
//                         best_j = j;
//                         best_j_diff = j_diff;
//                     }
//                 }
//             }
//             if best_j_diff != 1_000_000_000 {
//                 hako_indexes[best_j] = i;
//             }
//         }
//     }
// }
