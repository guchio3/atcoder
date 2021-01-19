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
        n: usize,
        mut a: [i64; n]
    }
    let mut tmpa: Vec<i64> = vec![0];
    tmpa.append(&mut a);
    a = tmpa;
    a.push(0);
    let mut res = 0;
    let mut bef_a_i = 0;
    for i in 1..a.len() {
        let a_i = a[i];
        res += (a_i - bef_a_i).abs();
        bef_a_i = a_i;
    }
    let mut bef_a_i = a[1];
    let mut bef_bef_a_i = a[0];
    for i in 2..a.len() {
        let a_i = a[i];
        println!(
            "{}",
            res - (bef_bef_a_i - bef_a_i).abs() - (a_i - bef_a_i).abs() + (bef_bef_a_i - a_i).abs()
        );
        bef_bef_a_i = bef_a_i;
        bef_a_i = a_i;
    }
}

// fn main() {
//     input! {
//         n: usize,
//         mut a: [i64; n]
//     }
//     a.push(0);
//     let mut res = 0;
//     let mut bef_a_i = 0;
//     for &a_i in a.iter() {
//         res += (a_i - bef_a_i).abs();
//         bef_a_i = a_i;
//     }
//     println!("{}", res - a[0].abs() - (a[1] - a[0]).abs() + a[1].abs());
//     let mut bef_a_i = a[1];
//     let mut bef_bef_a_i = a[0];
//     for i in 2..n {
//         let a_i = a[i];
//         println!(
//             "{}",
//             res - (bef_bef_a_i - bef_a_i).abs() - (a_i - bef_a_i).abs() + (bef_bef_a_i - a_i).abs()
//         );
//         bef_bef_a_i = bef_a_i;
//         bef_a_i = a_i;
//     }
//     println!(
//         "{}",
//         res - a[n - 1].abs() - (a[n - 1] - a[n - 2]).abs() + a[n - 2].abs()
//     );
// }

// fn main() {
//     input! {
//         n: usize,
//         mut a: [i64; n]
//     }
//     let mut a_copy: Vec<i64> = a.iter().map(|x| *x).collect();
//     a_copy.sort();
//     let a_min = a.iter().min().unwrap();
//     let a_max = a.iter().max().unwrap();
//     for &a_i in a.iter() {
//         if a_i == *a_min {
//             println!("{}", (max(a_copy[n - 1], 0) - min(0, a_copy[1])) * 2);
//         } else if a_i == *a_max {
//             println!("{}", (max(a_copy[n - 2], 0) - min(0, a_copy[0])) * 2);
//         } else {
//             println!("{}", (max(a_copy[n - 1], 0) - min(0, a_copy[0])) * 2);
//         }
//     }
// }
