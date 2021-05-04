#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use ordered_float::NotNan;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn check(x: usize, abcde: &Vec<(usize, usize, usize, usize, usize)>) -> bool {
    let mut pattern_set = HashSet::new();
    for &abcde_i in abcde.iter() {
        let mut pattern = 0;
        pattern += ((abcde_i.0 >= x) as usize) << 4;
        pattern += ((abcde_i.1 >= x) as usize) << 3;
        pattern += ((abcde_i.2 >= x) as usize) << 2;
        pattern += ((abcde_i.3 >= x) as usize) << 1;
        pattern += ((abcde_i.4 >= x) as usize) << 0;
        pattern_set.insert(pattern);
    }

    for &x in pattern_set.iter() {
        for &y in pattern_set.iter() {
            for &z in pattern_set.iter() {
                if x | y | z == 31 {
                    return true;
                }
            }
        }
    }
    false
}

fn main() {
    input! {
        n: usize,
        abcde: [(usize, usize, usize, usize, usize); n]
    }
    let (mut l, mut r) = (0, 1_000_000_001);
    while r - l > 1 {
        let x = (l + r) / 2;
        if check(x, &abcde) {
            l = x;
        } else {
            r = x;
        }
    }
    println!("{}", l);
}

// fn main() {
//     input! {
//         n: usize,
//         abcde: [(usize, usize, usize, usize, usize); n]
//     }
//     let mut reses = vec![];
//     for i in 0..n {
//         let abcde_i = abcde[i];
//         reses.push(abcde_i);
//         if reses.len() < 4 {
//             continue;
//         } else {
//             let mut best_j = 5;
//             let mut best_score = 0;
//             for j in 0..4 {
//                 let mut scores = vec![0, 0, 0, 0, 0];
//                 for k in 0..4 {
//                     if j == k {
//                         continue;
//                     }
//                     scores[0] = max(scores[0], reses[k].0);
//                     scores[1] = max(scores[1], reses[k].1);
//                     scores[2] = max(scores[2], reses[k].2);
//                     scores[3] = max(scores[3], reses[k].3);
//                     scores[4] = max(scores[4], reses[k].4);
//                 }
//                 let score: usize = scores.iter().map(|x| *x).min().unwrap();
//                 if score > best_score {
//                     best_j = j;
//                     best_score = score;
//                 }
//             }
//             if best_j == 5 {
//                 panic!();
//             }
//             let mut tmp_reses = vec![];
//             for j in 0..4 {
//                 if j != best_j {
//                     tmp_reses.push(reses[j]);
//                 }
//             }
//             reses = tmp_reses;
//         }
//     }
//     let mut scores = vec![0, 0, 0, 0, 0];
//     for k in 0..3 {
//         scores[0] = max(scores[0], reses[k].0);
//         scores[1] = max(scores[1], reses[k].1);
//         scores[2] = max(scores[2], reses[k].2);
//         scores[3] = max(scores[3], reses[k].3);
//         scores[4] = max(scores[4], reses[k].4);
//     }
//     let score: usize = scores.iter().map(|x| *x).min().unwrap();
//     println!("{}", score);
// }

// fn main() {
//     input! {
//         n: usize,
//         abcde: [(usize, usize, usize, usize, usize); n]
//     }
//     let mut best_info = vec![];
//     for k in 0..5 {
//         best_info.push(((3001, 3001), 0, vec![0, 0, 0, 0, 0]));
//     }
//     let mut best_score: usize = 0;
//     for i in 0..n {
//         for j in i + 1..n {
//             let (a_i, b_i, c_i, d_i, e_i) = abcde[i];
//             let (a_j, b_j, c_j, d_j, e_j) = abcde[j];
//             let scores = vec![
//                 max(a_i, a_j),
//                 max(b_i, b_j),
//                 max(c_i, c_j),
//                 max(d_i, d_j),
//                 max(e_i, e_j),
//             ];
//             for k in 0..5 {
//                 if scores[k] > best_info[k].1 {
//                     best_info[k] = ((i, j), scores[k], scores.clone());
//                 }
//             }
//         }
//     }
//     let mut last_best_score: usize = 0;
//     for k in 0..5 {
//         for i in 0..n {
//             if i == best_info[k].0 .0 || i == best_info[k].0 .1 {
//                 continue;
//             }
//             let (a_i, b_i, c_i, d_i, e_i) = abcde[i];
//             let scores = vec![
//                 max(best_info[k].2[0], a_i),
//                 max(best_info[k].2[1], b_i),
//                 max(best_info[k].2[2], c_i),
//                 max(best_info[k].2[3], d_i),
//                 max(best_info[k].2[4], e_i),
//             ];
//             let score: usize = scores.into_iter().min().unwrap();
//             if score > last_best_score {
//                 last_best_score = score;
//             }
//         }
//     }
//     println!("{}", last_best_score);
// }

// fn main() {
//     input! {
//         n: usize,
//         abcde: [(usize, usize, usize, usize, usize); n]
//     }
//     let mut best_index = (3001, 3001);
//     let mut best_scores = vec![0, 0, 0, 0, 0];
//     let mut best_score: usize = 0;
//     for i in 0..n {
//         for j in i + 1..n {
//             let (a_i, b_i, c_i, d_i, e_i) = abcde[i];
//             let (a_j, b_j, c_j, d_j, e_j) = abcde[j];
//             let scores = vec![
//                 max(a_i, a_j),
//                 max(b_i, b_j),
//                 max(c_i, c_j),
//                 max(d_i, d_j),
//                 max(e_i, e_j),
//             ];
//             let score: usize = scores.iter().map(|x| *x).min().unwrap();
//             if score >= best_score {
//                 println!("{:?}", scores);
//                 best_index = (i, j);
//                 best_scores = scores.clone();
//                 best_score = score;
//             }
//         }
//     }
//     let mut last_best_score: usize = 0;
//     for i in 0..n {
//         if i == best_index.0 || i == best_index.1 {
//             continue;
//         }
//         let (a_i, b_i, c_i, d_i, e_i) = abcde[i];
//         let scores = vec![
//             max(best_scores[0], a_i),
//             max(best_scores[1], b_i),
//             max(best_scores[2], c_i),
//             max(best_scores[3], d_i),
//             max(best_scores[4], e_i),
//         ];
//         let score: usize = scores.into_iter().min().unwrap();
//         if score > last_best_score {
//             println!("{}", i);
//             last_best_score = score;
//         }
//     }
//     println!("{}", last_best_score);
// }
