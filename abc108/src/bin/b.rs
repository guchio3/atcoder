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
        x1: i64, y1: i64, x2: i64, y2: i64
    }
    let x_diff = x2 - x1;
    let y_diff = y2 - y1;
    let x3 = x2 - y_diff;
    let y3 = y2 + x_diff;
    let x4 = x1 - y_diff;
    let y4 = y1 + x_diff;
    println!("{} {} {} {}", x3, y3, x4, y4);
}

// fn main() {
//     input! {
//         x1: i64, y1: i64, x2: i64, y2: i64
//     }
//     if x1 == x2 {
//         let length = (y1 - y2).abs();
//         if y1 > y2 {
//             println!("{} {} {} {}", x2 + length, y2, x2 + length, y1);
//         } else {
//             println!("{} {} {} {}", x2 - length, y2, x2 - length, y1);
//         }
//     } else {
//         let length = (x1 - x2).abs();
//         if x1 > x2 {
//             println!("{} {} {} {}", x2, y2 - length, x1, y2 - length);
//         } else {
//             println!("{} {} {} {}", x1, y2 - length, x2, y2 - length);
//         }
//     }
// }
