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
        mut b: [usize; n]
    }
    let mut reses = vec![];
    for _i in 0..n {
        let mut jj = 0;
        for j in (0..b.len()).rev() {
            if b[j] == j + 1 {
                jj = j;
                break;
            }
        }
        if b[jj] == jj + 1 {
            reses.push(jj + 1);
            b.remove(jj);
        } else {
            println!("-1");
            return;
        }
    }
    println!("{}", reses.into_iter().rev().join(" "));
}

// fn main() {
//     input! {
//         n: usize,
//         b: [usize; n]
//     }
//     let mut reses = vec![];
//     let mut bef_b_i = 0;
//     let mut tmp_b = vec![];
//     for i in 0..n {
//         let b_i = b[i];
//         if b_i >= bef_b_i {
//             tmp_b.push(b_i);
//         } else {
//             let bef_tmp_b_j = 0;
//             for tmp_b_j in tmp_b {
//                 if tmp_b_j < bef_tmp_b_j || (bef_tmp_b_j - tmp_b_j) > 2 {
//                     println!("-1");
//                     return;
//                 } else {
//                     if tmp_b_j == bef_tmp_b_j {
//                         reses.push(tmp_b_j);
//                     } else {
//                         reses.push();
//                     }
//                 }
//                 bef_tmp_b_j = tmp_b_j;
//             }
//             tmp_b = vec![];
//         }
//     }
//     println!("{}", reses.into_iter().join(" "));
// }
