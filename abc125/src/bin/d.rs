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
        a: [i64; n]
    }
    let mut minus_cnt = 0;
    let mut abs_sum = 0;
    let mut min_abs = 1_000_000_000;
    for i in 0..n {
        let a_i = a[i];
        minus_cnt += (a_i < 0) as usize;
        let a_i_abs = a_i.abs();
        abs_sum += a_i_abs;
        min_abs = min(min_abs, a_i_abs);
    }
    if minus_cnt % 2 == 0 {
        println!("{}", abs_sum);
    } else {
        println!("{}", abs_sum - min_abs * 2);
    }
}

// fn main() {
//     input! {
//         n: usize,
//         a: [i64; n]
//     }
//     let mut is_diff_sign = vec![false; n];
//     let mut bef_a_i = 0;
//     for i in 0..a.len() {
//         let a_i = a[i];
//         if a_i != 0 && bef_a_i != 0 && ((a_i > 0) != (bef_a_i > 0)) {
//             is_diff_sign[i] = true;
//         }
//         bef_a_i = a_i;
//     }
// }
