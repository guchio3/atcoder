#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        n: usize,
        xl: [(i64, i64); n],
    }
    let mut start_end = vec![];
    for i in 0..n {
        let (x_i, l_i) = xl[i];
        start_end.push((x_i - l_i, x_i + l_i));
    }
    start_end.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());
    let mut res = 0;
    let mut last_last = -10_000_000_000;
    for (s, e) in start_end {
        if s >= last_last {
            res += 1;
            last_last = e;
        }
    }
    println!("{}", res);
}

// fn main() {
//     input! {
//         n: usize,
//         xl: [(usize, usize); n],
//     }
//     let mut se = vec![];
//     for &xl_i in xl.iter() {
//         let x = xl_i.0;
//         let l = xl_i.1;
//         let s;
//         if x < l {
//             s = 0;
//         } else {
//             s = x - l;
//         }
//         let e = x + l;
//
//         se.push((s, e));
//     }
//     se.sort_by(|se1, se2| se2.1.partial_cmp(&se1.1).unwrap());
// }
