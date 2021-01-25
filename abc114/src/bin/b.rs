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
        s: Chars
    }
    let mut res: i64 = 1_000_000_000;
    for i in 2..s.len() {
        let num: i64 = s[(i - 2)..=i].into_iter().join("").parse().unwrap();
        res = min(res, (753 - num).abs());
    }
    println!("{}", res);
}

// fn main() {
//     input! {
//         s: Chars
//     }
//     let mut tmp = VecDeque::new();
//     tmp.push_back(s[0]);
//     tmp.push_back(s[1]);
//     let mut res: i64 = 1_000_000_000;
//     for i in 2..s.len() {
//         tmp.push_back(s[i]);
//         let num: i64 = tmp.iter().map(|x| *x).join("").parse().unwrap();
//         res = min(res, (753 - num).abs());
//         tmp.pop_front();
//     }
//     println!("{}", res);
// }
