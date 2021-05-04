#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::{gcd, lcm, Integer};
use num::traits::{NumAssign, PrimInt};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n]
    }
    if n % 2 == 0 {
        let mut a = vec![];
        let mut b = vec![];
        for i in 0..n {
            a.push(ab[i].0);
            b.push(ab[i].1);
        }
        a.sort();
        b.sort();
        println!("{}", b[n / 2] - a[n / 2] + b[n / 2 - 1] - a[n / 2 - 1] + 1);
    } else {
        let mut nums = vec![];
        for (a_i, b_i) in ab {
            nums.push((a_i, 0));
            nums.push((b_i, 1));
        }
        nums.sort();
        let mut a_cnt = 0;
        let mut b_cnt = 0;
        let mut a_start = 0;
        let mut b_end = 0;
        for num in nums {
            if num.1 == 0 {
                a_cnt += 1;
            } else {
                b_cnt += 1;
            }
            if a_cnt > n / 2 {
                if a_start == 0 {
                    a_start = num.0;
                }
                if b_cnt > n / 2 {
                    b_end = num.0;
                    break;
                }
            }
        }
        println!("{}", b_end - a_start + 1);
    }
}

// fn main() {
//     input! {
//         n: usize,
//         ab: [(usize, usize); n]
//     }
//     if n % 2 == 0 {
//         let mut a = vec![];
//         let mut b = vec![];
//         for i in 0..n {
//             a.push(ab[i].0);
//             b.push(ab[i].1);
//         }
//         a.sort();
//         b.sort();
//         println!("{}", b[0] - a[0] + b[n - 1] - a[n - 1] + 1);
//     } else {
//         let mut nums = vec![];
//         for (a_i, b_i) in ab {
//             nums.push((a_i, 0));
//             nums.push((b_i, 1));
//         }
//         nums.sort();
//         let mut a_cnt = 0;
//         let mut b_cnt = 0;
//         let mut a_start = 0;
//         let mut b_end = 0;
//         for num in nums {
//             if num.1 == 0 {
//                 a_cnt += 1;
//             } else {
//                 b_cnt += 1;
//             }
//             if a_cnt > n / 2 {
//                 if a_start == 0 {
//                     a_start = num.0;
//                 }
//                 if b_cnt > n / 2 {
//                     b_end = num.0;
//                     break;
//                 }
//             }
//         }
//         println!("{}", b_end - a_start + 1);
//     }
// }
