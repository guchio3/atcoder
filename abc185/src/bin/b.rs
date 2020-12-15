#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: i64, m: usize, t: i64,
        ab: [(i64, i64); m]
    }

    let mut tmp_n = n;
    let mut bef_start: i64 = 0;
    for ab_i in ab {
        tmp_n = tmp_n - (ab_i.0 - bef_start);
        if tmp_n <= 0 {
            println!("No");
            return;
        }
        tmp_n = min(n, tmp_n + ab_i.1 - ab_i.0);
        bef_start = ab_i.1;
    }
    tmp_n = tmp_n - (t - bef_start);
    if tmp_n <= 0 {
        println!("No");
        return;
    }
    println!("Yes");
}


// fn main() {
//     input! {
//         n: usize, m: usize, t: usize,
//         ab: [(usize, usize); m]
//     }
// 
//     let mut tmp_n = n;
//     let mut ab_i = 0;
//     for t_i in 0..t {
//         if ab_i < ab.len() {
//             if ab[ab_i].1 <= t_i {
//                 ab_i += 1;
//             }
//             if ab_i < ab.len() && ab[ab_i].0 <= t_i {
//                 tmp_n = min(n, tmp_n + 1);
//             } else {
//                 tmp_n -= 1;
//                 if tmp_n == 0 {
//                     println!("No");
//                     return;
//                 }
//             }
//         } else {
//             tmp_n -= 1;
//             if tmp_n == 0 {
//                 println!("No");
//                 return;
//             }
//         }
//     }
//     println!("Yes");
// }
