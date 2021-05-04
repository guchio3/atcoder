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
        t: usize,
        xypq: [(i64, i64, i64, i64); t]
    }
    for (xi, yi, pi, qi) in xypq {}
}

// fn main() {
//     input! {
//         t: usize,
//         xypq: [(i64, i64, i64, i64); t]
//     }
//     for (xi, yi, pi, qi) in xypq {
//         let mut is_in_b = false;
//         let mut is_awake = false;
//         let mut n = 0;
//         let mut m = 0;
//         let mut seen_in_past = HashSet::new();
//         // let mut cnt = 0;
//         loop {
//             let n_next_time;
//             let m_next_time;
//             if is_in_b {
//                 n_next_time = (2 * xi + 2 * yi) * n + xi + yi;
//             } else {
//                 n_next_time = (2 * xi + 2 * yi) * n + xi;
//             }
//             if is_awake {
//                 m_next_time = (pi + qi) * (m + 1);
//             } else {
//                 m_next_time = (pi + qi) * m + pi;
//             }
//             // println!("n_next_time: {} m_next_time {}", n_next_time, m_next_time);
//             if n_next_time <= m_next_time {
//                 if is_in_b {
//                     let next_step = (m_next_time - n_next_time) / (2 * xi + 2 * yi);
//                     if next_step == 0 {
//                         n += 1;
//                     } else {
//                         n += next_step;
//                     }
//                 }
//                 is_in_b = !is_in_b;
//             }
//             if n_next_time >= m_next_time {
//                 if is_awake {
//                     let next_step = (n_next_time - m_next_time) / (pi + qi);
//                     if next_step == 0 {
//                         m += 1;
//                     } else {
//                         m += next_step;
//                     }
//                 }
//                 is_awake = !is_awake;
//             }
//             if is_in_b && is_awake {
//                 let res;
//                 if n_next_time <= m_next_time {
//                     res = n_next_time;
//                 } else {
//                     res = m_next_time;
//                 }
//                 println!("{}", res);
//                 break;
//             }
//             let time_diff = n_next_time - m_next_time;
//             if seen_in_past.contains(&time_diff) {
//                 println!("infinity");
//                 break;
//             } else {
//                 seen_in_past.insert(time_diff);
//             }
//             // if cnt > 1000_000 {
//             //     break;
//             // }
//             // cnt += 1;
//         }
//     }
// }
