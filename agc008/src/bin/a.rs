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
        x: i64, y: i64
    }
    let mut res = 1_000_000_000_000;
    if y >= x {
        res = min(res, y - x);
    }
    if y >= -x {
        res = min(res, y + x + 1);
    }
    if -y >= x {
        res = min(res, -y - x + 1);
    }
    if -y >= -x {
        res = min(res, -y + x + 2);
    }

    println!("{}", res);
}

// fn main() {
//     input! {
//         x: i64, y: i64
//     }
//     let res;
//     if x.abs() == y.abs() {
//         res = (x * y < 0) as i64;
//     } else if x == 0 {
//         if y > 0 {
//             res = y;
//         } else {
//             res = -y + 1;
//         }
//     } else if x > 0 {
//         if y >= x {
//             res = y - x;
//         } else {
//             if x >= y.abs() {
//                 res = min(x - y + 2, y + x + 1);
//             } else {
//                 res = -x - y + 1;
//             }
//         }
//     } else {
//         if x >= y {
//             res = x - y + 2;
//         } else {
//             if -x >= y.abs() {
//                 res = min(-y - x + 1, y - x);
//             } else {
//                 res = y + x + 1;
//             }
//         }
//     }
//
//     println!("{}", res);
// }
