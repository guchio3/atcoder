#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        a: usize, b: usize, c: usize
    }

    let mut dp = vec![vec![vec![0; 101]; 101]; 101];
    for i in (0..100).into_iter().rev() {
        for j in (0..100).into_iter().rev() {
            for k in (0..100).into_iter().rev() {
                dp[i][j][k] = dp[][][];
            }
        }
    }

    println!("{}", dp[a][b][c]);
}

// fn main() {
//     input! {
//         a: i64, b: i64, c: i64
//     }
//     let mut res = 0.;
//     let prob: f64 = 1. / 3.;
//
//     // nCr 保存
//     let mut dp = vec![vec![-1.; 300]; 300];
//     for i in 0..100 {
//         for j in 0..100 {
//             let count = (100 + i + j - a - b - c) as i32;
//             if i >= b && j >= c {
//                 res += (count as f64) * prob.powi(count) * ;
//             }
//             if i >= a && j >= c {
//                 res += (count as f64) * prob.powi(count);
//             }
//             if i >= a && j >= b {
//                 res += (count as f64) * prob.powi(count);
//             }
//         }
//     }
//
//     println!("{}", res);
// }
