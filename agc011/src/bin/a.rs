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
        n: usize, c: usize, k: usize,
        mut t: [usize; n]
    }
    t.sort();

    let mut res = 0;
    let mut i = 0;
    let mut j = 0;
    while i < n {
        if t[j] + k < t[i] || i - j >= c {
            res += 1;
            j = i;
        }
        i += 1;
    }

    while j < n {
        if t[j] + k < t[n - 1] || n - 1 - j >= c || j >= n - 1 {
            res += 1;
        }
        j += 1;
    }

    println!("{}", res);
}

// fn main() {
//     input! {
//         n: usize, c: usize, k: usize,
//         mut t: [usize; n]
//     }
//     t.sort();
//
//     let mut res = 0;
//     let mut waiting_queue = VecDeque::new();
//     for t_i in t {
//         if waiting_queue.len() > 0 {
//             while waiting_queue.len() > 0 && *waiting_queue.front().unwrap() + k < t_i {
//                 let start_time = *waiting_queue.front().unwrap();
//                 for _i in 0..min(c, waiting_queue.len()) {
//                     let tmp_start_time = *waiting_queue.front().unwrap();
//                     if tmp_start_time <= start_time + k {
//                         waiting_queue.pop_front();
//                     }
//                 }
//                 res += 1;
//             }
//         }
//         waiting_queue.push_back(t_i);
//     }
//     while waiting_queue.len() > 0 {
//         let start_time = *waiting_queue.front().unwrap();
//         for _i in 0..min(c, waiting_queue.len()) {
//             let tmp_start_time = *waiting_queue.front().unwrap();
//             if tmp_start_time <= start_time + k {
//                 waiting_queue.pop_front();
//             }
//         }
//         res += 1;
//     }
//
//     println!("{}", res);
// }
