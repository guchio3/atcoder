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
        a: [i64; n]
    }
    let mut res = a[0];
    for a_i in a {
        res = gcd(res, a_i);
    }
    println!("{}", res);
}

// fn main() {
//     input! {
//         n: usize,
//         a: [i64; n]
//     }
//     let mut bh = BinaryHeap::new();
//     for a_i in a {
//         bh.push(a_i);
//     }
//
//     while bh.len() > 1 {
//         let mut largest = bh.pop().unwrap();
//         let second_largest = bh.pop().unwrap();
//         largest -= second_largest * (largest / second_largest);
//         bh.push(second_largest);
//         if largest >= 1 {
//             bh.push(largest);
//         }
//     }
//     println!("{}", bh.peek().unwrap());
// }
