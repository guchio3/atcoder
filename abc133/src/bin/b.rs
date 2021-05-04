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
        n: usize, d: usize,
        x: [[i64; d]; n]
    }
    let mut res = 0;
    for i in 0..n {
        for j in i + 1..n {
            let mut ij_sum = 0;
            for k in 0..d {
                ij_sum += (x[i][k] - x[j][k]).pow(2);
            }
            for i in 1..=ij_sum {
                if i * i == ij_sum {
                    res += 1;
                }
                if i * i >= ij_sum {
                    break;
                }
            }
        }
    }
    println!("{}", res)
}

// fn main() {
//     input! {
//         n: usize, d: usize,
//         x: [[i64; d]; n]
//     }
//     let mut res = 0;
//     for i in 0..n {
//         for j in i + 1..n {
//             let mut ij_sum = 0;
//             for k in 0..d {
//                 ij_sum += (x[i][k] - x[j][k]).pow(2);
//             }
//             if ij_sum == ((ij_sum as f64).powf(0.5) as i64).pow(2) {
//                 res += 1;
//             }
//         }
//     }
//     println!("{}", res)
// }
