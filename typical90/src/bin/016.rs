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

fn main() {
    input! {
        n: usize,
        a: usize, b: usize, c: usize
    }
    let mut candidates = vec![];
    for i in 0..=9999 {
        for j in 0..=(9999 - i) {
            let num = a * i + b * j;
            if num <= n && (n - num) % c == 0 && (i + j + (n - num) / c) <= 9999 {
                candidates.push(i + j + (n - num) / c);
            }
        }
    }
    candidates.sort();
    println!("{}", candidates[0]);
}

// fn main() {
//     input! {
//         n: usize,
//         a: usize, b: usize, c: usize
//     }
//     let mut candidates = vec![];
//     for i in 0..=9999 {
//         for j in 0..=(9999 - i) {
//             let num = a * i + b * j;
//             if num <= n {
//                 candidates.push((i + j, num));
//             }
//         }
//     }
//
//     let mut reses = vec![];
//     for (coin_num, price) in candidates {
//         if (n - price) % c == 0 {
//             let rest_num = (n - price) / c;
//             reses.push(coin_num + rest_num);
//         }
//     }
//
//     reses.sort();
//     println!("{}", reses[0]);
// }
