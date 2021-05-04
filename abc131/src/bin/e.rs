#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

// fn main() {
//     input! {
//         n: usize, k: usize
//     }
//     if k > (n - 1) * (n - 2) / 2 {
//         println!("-1");
//     } else {
//         let ma = (n - 1) * (n - 2) / 2;
//         let mut res = vec![];
//         for i in 1..=n - 1 {
//             res.push((i, n));
//         }
//         let rem = ma - k;
//         let mut ci = 1;
//         let mut cj = 2;
//         for _ in 0..rem {
//             res.push((ci, cj));
//             cj += 1;
//             if cj == n {
//                 ci += 1;
//                 cj = ci + 1;
//             }
//         }
//         println!("{}", res.len());
//         for res_i in res {
//             println!("{} {}", res_i.0, res_i.1);
//         }
//     }
// }

fn main() {
    input! {
        n: usize, k: usize
    }
    if k > (n - 1) * (n - 2) / 2 {
        println!("-1");
    } else {
        let mut reses = vec![];
        let num = (n - 1) * (n - 2) / 2 - k;
        for i in 0..n - 1 {
            reses.push((1, i + 2));
        }
        let mut cnt = 0;
        'outer: for i in 2..=n {
            for j in i + 1..=n {
                if cnt >= num {
                    break 'outer;
                } else {
                    reses.push((i, j));
                    cnt += 1;
                }
            }
        }
        println!("{}", reses.len());
        for res in reses {
            println!("{} {}", res.0, res.1);
        }
    }
}

// fn main() {
//     input! {
//         n: usize, k: usize
//     }
//     let mut combination_map = vec![0; 101];
//     combination_map[2] = 1;
//     for i in 3..100 {
//         combination_map[i] = combination_map[i - 1] * i / (i - 2);
//     }
//
//     for i in 0..(1 << (n + 1) * n / 2) {
//         let mut dims = vec![0; n];
//         let mut edge_nums = 0;
//         let mut reses = vec![];
//         for j in 0..n {
//             for k in 0..n {
//                 if i & (1 << (j * n + k)) > 0 {
//                     dims[j] += 1;
//                     dims[k] += 1;
//                     edge_nums += 1;
//                     reses.push((j + 1, k + 1));
//                 }
//             }
//         }
//         let mut all_dims = 0;
//         for j in 0..n {
//             all_dims += combination_map[dims[j]];
//         }
//         if all_dims == k {
//             println!("{}", edge_nums);
//             for res in reses {
//                 println!("{} {}", res.0, res.1);
//             }
//             return;
//         }
//     }
//     println!("-1");
// }
