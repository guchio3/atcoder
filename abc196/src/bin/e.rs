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
        at: [(i64, usize); n],
        q: usize,
        x: [i64; q]
    }
    let mut low = 0;
    let mut high = 0;
    let mut add = 0;
    for (a_i, t_i) in at {

    }
}

// fn main() {
//     input! {
//         n: usize,
//         at: [(i64, usize); n],
//         q: usize,
//         x: [i64; q]
//     }
//     let mut bef_t_i = 0;
//     let mut bef_a_i = 0;
//     let mut new_at = vec![];
//     for (a_i, t_i) in at {
//         if t_i == 1 {
//             bef_a_i += 1;
//             bef_t_i = t_i;
//         } else if t_i == bef_t_i {
//             match t_i {
//                 2 => {
//                     bef_a_i = max(bef_a_i, a_i);
//                 }
//                 3 => {
//                     bef_a_i = min(bef_a_i, a_i);
//                 }
//                 _ => {
//                     panic!();
//                 }
//             }
//         } else {
//             new_at.push((bef_a_i, bef_t_i));
//             bef_t_i = t_i;
//             bef_a_i = a_i;
//         }
//     }
//     new_at.push((bef_a_i, bef_t_i));
//
//     let mut bef_t_i = 0;
//     let mut bef_a_i = 0;
//     let mut newnew_at = vec![];
//     for (a_i, t_i) in new_at {
//         match (bef_t_i, t_i) {
//             (1, 1) => {
//                 bef_a_i += a_i;
//             }
//             (2, 2) => {
//                 bef_a_i = max(bef_a_i, a_i);
//             }
//             (3, 3) => {
//                 bef_a_i = min(bef_a_i, a_i);
//             }
//             (1, 2) => {}
//             (2, 1) => {
//                 bef_a_i += a_i;
//             }
//             (1, 3) => {}
//             (3, 1) => {
//                 bef_a_i += a_i;
//             }
//             (2, 3) => {
//                 if bef_a_i > a_i {
//                     bef_a_i = a_i;
//                 } else {
//                     newnew_at.push((bef_a_i, bef_t_i));
//                 }
//             }
//             (3, 2) => {
//                 if bef_a_i < a_i {
//                     bef_a_i = bef_a_i;
//                 } else {
//                     newnew_at.push((bef_a_i, bef_t_i));
//                 }
//             }
//             _ => {
//                 bef_a_i = a_i;
//             }
//         }
//         bef_t_i = t_i;
//     }
//
//     for i in 0..q {
//         let xi = x[i];
//     }
// }
