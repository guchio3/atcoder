#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

// fn main() {
//     input! {
//         n: usize,
//         ab: [(i64, i64); n]
//     }
//     let mut a_plus_b_vec = vec![];
//     for i in 0..n {
//         let (a_i, b_i) = ab[i];
//         a_plus_b_vec.push((a_i + b_i, i));
//     }
//     a_plus_b_vec.sort_by(|x, y| y.partial_cmp(&x).unwrap());
//     let mut sum = 0;
//     let mut b_sum = 0;
//     for i in 0..n {
//         if i % 2 == 0 {
//             sum += a_plus_b_vec[i].0;
//         }
//         b_sum += ab[a_plus_b_vec[i].1].1;
//     }
//     println!("{}", sum - b_sum);
// }

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n]
    }
    let mut a = BinaryHeap::new();
    let mut b = BinaryHeap::new();
    for i in 0..n {
        a.push((ab[i].0 + ab[i].1, i));
        b.push((ab[i].1 + ab[i].0, i));
    }

    let mut used = vec![false; n];
    let mut a_sum: i64 = 0;
    let mut b_sum: i64 = 0;
    for i in 0..n {
        if i % 2 == 0 {
            loop {
                let a_i = a.pop().unwrap();
                if !used[a_i.1] {
                    used[a_i.1] = true;
                    a_sum += ab[a_i.1].0;
                    // println!("{:?}", a_i);
                    break;
                }
            }
        } else {
            loop {
                let b_i = b.pop().unwrap();
                if !used[b_i.1] {
                    used[b_i.1] = true;
                    b_sum += ab[b_i.1].1;
                    // println!("{:?}", b_i);
                    break;
                }
            }
        }
    }
    println!("{}", a_sum - b_sum);
}

// fn main() {
//     input! {
//         n: usize,
//         ab: [(i64, i64); n]
//     }
//     let mut a = BinaryHeap::new();
//     let mut b = BinaryHeap::new();
//     for i in 0..n {
//         a.push((ab[i].0, i));
//         b.push((ab[i].1, i));
//     }
//
//     let mut used = vec![false; n];
//     let mut a_sum: i64 = 0;
//     let mut b_sum: i64 = 0;
//     for i in 0..n {
//         if i % 2 == 0 {
//             loop {
//                 let a_i = a.pop().unwrap();
//                 if !used[a_i.1] {
//                     used[a_i.1] = true;
//                     a_sum += a_i.0;
//                     break;
//                 }
//             }
//         } else {
//             loop {
//                 let b_i = b.pop().unwrap();
//                 if !used[b_i.1] {
//                     used[b_i.1] = true;
//                     b_sum += b_i.0;
//                     break;
//                 }
//             }
//         }
//     }
//     println!("{}", a_sum - b_sum);
// }
